// SPDX-License-Identifier: GPL-2.0
// SPDX-FileCopyrightText: Copyright Collabora 2024

//! Dump the GPU state to a file, so we can figure out what went wrong if it
//! crashes.
//!
//! The dump is comprised of the following sections:
//!
//! Registers,
//! BoData
//!
//! Each section is preceded by a header that describes it. Most importantly,
//! each header starts with a magic number that should be used by userspace to
//! when decoding.
//!

use core::mem;
use core::ptr::NonNull;

use alloc::DumpAllocator;
use kernel::bindings;
use kernel::prelude::*;

use crate::regs;
use crate::regs::GpuRegister;

const MAJOR: u32 = 1;
const MINOR: u32 = 0;

// PANT
const MAGIC: u32 = 0x544e4150;

#[derive(Copy, Clone)]
#[repr(u32)]
enum HeaderType {
    /// A register dump
    Registers,
    /// The VM data,
    Vm,
    /// A dump of the firmware interface
    FirmwareInterface,
}

#[repr(C)]
pub(crate) struct DumpArgs {
    dev: *mut bindings::device,
    /// The slot for the job
    slot: i32,
    /// The active buffer objects
    bos: *mut *mut bindings::drm_gem_object,
    /// The number of active buffer objects
    bo_count: usize,
    /// The base address of the registers to use when reading.
    reg_base_addr: *mut core::ffi::c_void,
}

#[repr(C)]
pub(crate) struct Header {
    magic: u32,
    ty: HeaderType,
    size: u32,
    padding: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct RegisterDump {
    register: GpuRegister,
    value: u32,
}

/// The registers to dump
const REGISTERS: [GpuRegister; 18] = [
    regs::SHADER_READY_LO,
    regs::SHADER_READY_HI,
    regs::TILER_READY_LO,
    regs::TILER_READY_HI,
    regs::L2_READY_LO,
    regs::L2_READY_HI,
    regs::JOB_INT_MASK,
    regs::JOB_INT_STAT,
    regs::MMU_INT_MASK,
    regs::MMU_INT_STAT,
    regs::as_transtab_lo(0),
    regs::as_transtab_hi(0),
    regs::as_memattr_lo(0),
    regs::as_memattr_hi(0),
    regs::as_faultstatus(0),
    regs::as_faultaddress_lo(0),
    regs::as_faultaddress_hi(0),
    regs::as_status(0),
];

mod alloc {
    use core::ptr::NonNull;

    use kernel::bindings;
    use kernel::prelude::*;

    use crate::dump::Header;
    use crate::dump::HeaderType;
    use crate::dump::MAGIC;

    pub(crate) struct DumpAllocator {
        mem: NonNull<core::ffi::c_void>,
        pos: usize,
        capacity: usize,
    }

    impl DumpAllocator {
        pub(crate) fn new(size: usize) -> Result<Self> {
            if isize::try_from(size).unwrap() == isize::MAX {
                return Err(EINVAL);
            }

            // Let's cheat a bit here, since there is no Rust vmalloc allocator
            // for the time being.
            //
            // Safety: just a FFI call to alloc memory
            let mem = NonNull::new(unsafe {
                bindings::__vmalloc_noprof(
                    size.try_into().unwrap(),
                    bindings::GFP_KERNEL | bindings::GFP_NOWAIT | 1 << bindings::___GFP_NORETRY_BIT,
                )
            });

            let mem = match mem {
                Some(buffer) => buffer,
                None => return Err(ENOMEM),
            };

            // Ssfety: just a FFI call to zero out the memory
            unsafe { core::ptr::write_bytes(mem.as_ptr(), 0, size) };
            Ok(Self {
                mem,
                pos: 0,
                capacity: size,
            })
        }

        fn alloc_mem(&mut self, size: usize) -> Option<*mut u8> {
            assert!(size % 8 == 0, "Allocation size must be 8-byte aligned");
            if isize::try_from(size).unwrap() == isize::MAX {
                return None;
            } else if self.pos + size > self.capacity {
                kernel::pr_debug!("DumpAllocator out of memory");
                None
            } else {
                let offset = self.pos;
                self.pos += size;

                // Safety: we know that this is a valid allocation, so
                // dereferencing is safe. We don't ever return two pointers to
                // the same address, so we adhere to the aliasing rules. We make
                // sure that the memory is zero-initialized before being handed
                // out (this happens when the allocator is first created) and we
                // enforce a 8 byte alignment rule.
                Some(unsafe { self.mem.as_ptr().offset(offset as isize) as *mut u8 })
            }
        }

        pub(crate) fn alloc<T>(&mut self) -> Option<&mut T> {
            let mem = self.alloc_mem(core::mem::size_of::<T>())? as *mut T;
            // Safety: we uphold safety guarantees in alloc_mem(), so this is
            // safe to dereference.
            Some(unsafe { &mut *mem })
        }

        pub(crate) fn alloc_bytes(&mut self, num_bytes: usize) -> Option<&mut [u8]> {
            let mem = self.alloc_mem(num_bytes)?;

            // Safety: we uphold safety guarantees in alloc_mem(), so this is
            // safe to build a slice
            Some(unsafe { core::slice::from_raw_parts_mut(mem, num_bytes) })
        }

        pub(crate) fn alloc_header(&mut self, ty: HeaderType, size: u32) -> &mut Header {
            let hdr: &mut Header = self.alloc().unwrap();
            hdr.magic = MAGIC;
            hdr.ty = ty;
            hdr.size = size;
            hdr
        }

        pub(crate) fn is_end(&self) -> bool {
            self.pos == self.capacity
        }

        pub(crate) fn dump(self) -> (NonNull<core::ffi::c_void>, usize) {
            (self.mem, self.capacity)
        }
    }
}

fn dump_registers(alloc: &mut DumpAllocator, args: &DumpArgs) {
    let sz = core::mem::size_of_val(&REGISTERS);
    let header: &mut Header = alloc.alloc_header(HeaderType::Registers, sz.try_into().unwrap());

    // TODO: js_as_offset;
    for reg in &REGISTERS {
        let dumped_reg: &mut RegisterDump = alloc.alloc().unwrap();
        dumped_reg.register = *reg;
        dumped_reg.value = reg.read(args.reg_base_addr);
    }
}

fn dump_bo(alloc: &mut DumpAllocator, bo: &mut bindings::drm_gem_object) {
    let mut map = bindings::iosys_map::default();

    // Safety: we trust the kernel to provide a valid BO.
    let ret = unsafe { bindings::drm_gem_vmap_unlocked(bo, &mut map as _) };
    if ret != 0 {
        pr_warn!("Failed to map BO");
        return;
    }

    let sz = bo.size;

    // Safety: we know that the vaddr is valid and we know the BO size.
    let mapped_bo: &mut [u8] =
        unsafe { core::slice::from_raw_parts_mut(map.__bindgen_anon_1.vaddr as *mut _, sz) };

    let padding = (8 - bo.size % 8) % 8;
    let header = alloc.alloc_header(HeaderType::Vm, sz as u32);
    header.padding = padding as u16;

    let bo_data = alloc.alloc_bytes(sz + padding).unwrap();
    bo_data.copy_from_slice(&mapped_bo[..]);

    // Safety: BO is valid and was previously mapped.
    unsafe { bindings::drm_gem_vunmap_unlocked(bo, &mut map as _) };
}

/// Dumps the current state of the GPU to a file
///
/// # Safety
///
/// `Args` must be aligned and non-null.
/// All fields of `DumpArgs` must be valid.
#[no_mangle]
pub(crate) extern "C" fn panthor_core_dump(args: *const DumpArgs) -> core::ffi::c_int {
    assert!(!args.is_null());
    // Safety: we checked whether the pointer was null. It is assumed to be
    // aligned as per the safety requirements.
    let args = unsafe { &*args };
    // Safety: `args` is assumed valid as per the safety requirements.
    //
    // TODO: Ideally, we would use the safe GEM abstraction from the kernel
    // crate, but I see no way to create a drm::gem::ObjectRef from a
    // bindings::drm_gem_object. drm::gem::IntoGEMObject is only implemented for
    // drm::gem::Object, which means that new references can only be created
    // from a Rust-owned GEM object.
    //
    // It also has a has a `type Driver: drv::Driver` associated type, from
    // which it can access the `File` associated type. But not all GEM functions
    // take a file, though. For example, `drm_gem_vmap_unlocked` (used here)
    // does not.
    //
    // This associated type is a blocker here, because there is no actual
    // drv::Driver. We're only implementing a few functions in Rust.
    let mut bos = match Vec::with_capacity(args.bo_count, GFP_KERNEL) {
        Ok(bos) => bos,
        Err(_) => return ENOMEM.to_errno(),
    };
    for i in 0..args.bo_count {
        // Safety: `args` is assumed valid as per the safety requirements.
        // `bos` is a valid pointer to a valid array of valid pointers.
        let bo = unsafe { &mut **args.bos.add(i) };
        bos.push(bo, GFP_KERNEL).unwrap();
    }

    // let mut bos: Vec<&mut bindings::drm_gem_object> = (0..args.bo_count)
    // .map(|i| unsafe { &mut **args.bos.add(i) })
    // .collect();

    let mut file_size = core::mem::size_of::<Header>();
    file_size += REGISTERS.len() * core::mem::size_of::<RegisterDump>();

    for bo in &mut *bos {
        file_size += core::mem::size_of::<Header>();
        file_size += bo.size;
        let padding = ((8 - bo.size % 8) % 8) as u16;
        file_size += padding as usize;
    }

    // Everything must fit within this allocation, otherwise it was miscomputed.
    let mut alloc = match DumpAllocator::new(file_size) {
        Ok(alloc) => alloc,
        Err(e) => return e.to_errno(),
    };

    dump_registers(&mut alloc, &args);
    for bo in bos {
        dump_bo(&mut alloc, bo);
    }

    if alloc.is_end() {
        pr_warn!("DumpAllocator: wrong allocation size");
    }

    let (mem, size) = alloc.dump();
    // Safety: `mem` is a valid pointer to a valid allocation of `size` bytes.
    unsafe { bindings::dev_coredumpv(args.dev, mem.as_ptr(), size, bindings::GFP_KERNEL) };
    0
}
