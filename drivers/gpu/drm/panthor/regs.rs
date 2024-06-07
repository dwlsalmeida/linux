// SPDX-License-Identifier: GPL-2.0
// SPDX-FileCopyrightText: Copyright Collabora 2024
// SPDX-FileCopyrightText: (C) COPYRIGHT 2010-2022 ARM Limited. All rights reserved.

//! The registers for Panthor, extracted from panthor_regs.h

#![allow(unused_macros, unused_imports, dead_code)]

use kernel::bindings;

use core::ops::Add;
use core::ops::Shl;
use core::ops::Shr;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub(crate) struct GpuRegister(u64);

impl GpuRegister {
    pub(crate) fn read(&self, iomem: *const core::ffi::c_void) -> u32 {
        // Safety: `reg` represents a valid address
        unsafe {
            let addr = iomem.offset(self.0 as isize);
            bindings::readl_relaxed(addr as *const _)
        }
    }
}

impl Add for GpuRegister {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        GpuRegister(self.0 + rhs.0)
    }
}

impl Shl for GpuRegister {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        GpuRegister(self.0 << rhs.0)
    }
}

impl Shr for GpuRegister {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        GpuRegister(self.0 >> rhs.0)
    }
}

pub(crate) const fn bit(index: u64) -> u64 {
    1 << index
}
pub(crate) const fn genmask(high: u64, low: u64) -> u64 {
    ((1 << (high - low + 1)) - 1) << low
}

pub(crate) const GPU_ID: GpuRegister = GpuRegister(0x0);
pub(crate) const fn gpu_arch_major(x: u64) -> GpuRegister {
    GpuRegister((x) >> 28)
}
pub(crate) const fn gpu_arch_minor(x: u64) -> GpuRegister {
    GpuRegister((x) & genmask(27, 24) >> 24)
}
pub(crate) const fn gpu_arch_rev(x: u64) -> GpuRegister {
    GpuRegister((x) & genmask(23, 20) >> 20)
}
pub(crate) const fn gpu_prod_major(x: u64) -> GpuRegister {
    GpuRegister((x) & genmask(19, 16) >> 16)
}
pub(crate) const fn gpu_ver_major(x: u64) -> GpuRegister {
    GpuRegister((x) & genmask(15, 12) >> 12)
}
pub(crate) const fn gpu_ver_minor(x: u64) -> GpuRegister {
    GpuRegister((x) & genmask(11, 4) >> 4)
}
pub(crate) const fn gpu_ver_status(x: u64) -> GpuRegister {
    GpuRegister(x & genmask(3, 0))
}
pub(crate) const GPU_L2_FEATURES: GpuRegister = GpuRegister(0x4);
pub(crate) const fn gpu_l2_features_line_size(x: u64) -> GpuRegister {
    GpuRegister(1 << ((x) & genmask(7, 0)))
}
pub(crate) const GPU_CORE_FEATURES: GpuRegister = GpuRegister(0x8);
pub(crate) const GPU_TILER_FEATURES: GpuRegister = GpuRegister(0xc);
pub(crate) const GPU_MEM_FEATURES: GpuRegister = GpuRegister(0x10);
pub(crate) const GROUPS_L2_COHERENT: GpuRegister = GpuRegister(bit(0));
pub(crate) const GPU_MMU_FEATURES: GpuRegister = GpuRegister(0x14);
pub(crate) const fn gpu_mmu_features_va_bits(x: u64) -> GpuRegister {
    GpuRegister((x) & genmask(7, 0))
}
pub(crate) const fn gpu_mmu_features_pa_bits(x: u64) -> GpuRegister {
    GpuRegister(((x) >> 8) & genmask(7, 0))
}
pub(crate) const GPU_AS_PRESENT: GpuRegister = GpuRegister(0x18);
pub(crate) const GPU_CSF_ID: GpuRegister = GpuRegister(0x1c);
pub(crate) const GPU_INT_RAWSTAT: GpuRegister = GpuRegister(0x20);
pub(crate) const GPU_INT_CLEAR: GpuRegister = GpuRegister(0x24);
pub(crate) const GPU_INT_MASK: GpuRegister = GpuRegister(0x28);
pub(crate) const GPU_INT_STAT: GpuRegister = GpuRegister(0x2c);
pub(crate) const GPU_IRQ_FAULT: GpuRegister = GpuRegister(bit(0));
pub(crate) const GPU_IRQ_PROTM_FAULT: GpuRegister = GpuRegister(bit(1));
pub(crate) const GPU_IRQ_RESET_COMPLETED: GpuRegister = GpuRegister(bit(8));
pub(crate) const GPU_IRQ_POWER_CHANGED: GpuRegister = GpuRegister(bit(9));
pub(crate) const GPU_IRQ_POWER_CHANGED_ALL: GpuRegister = GpuRegister(bit(10));
pub(crate) const GPU_IRQ_CLEAN_CACHES_COMPLETED: GpuRegister = GpuRegister(bit(17));
pub(crate) const GPU_IRQ_DOORBELL_MIRROR: GpuRegister = GpuRegister(bit(18));
pub(crate) const GPU_IRQ_MCU_STATUS_CHANGED: GpuRegister = GpuRegister(bit(19));
pub(crate) const GPU_CMD: GpuRegister = GpuRegister(0x30);
const fn gpu_cmd_def(ty: u64, payload: u64) -> u64 {
    (ty) | ((payload) << 8)
}
pub(crate) const fn gpu_soft_reset() -> GpuRegister {
    GpuRegister(gpu_cmd_def(1, 1))
}
pub(crate) const fn gpu_hard_reset() -> GpuRegister {
    GpuRegister(gpu_cmd_def(1, 2))
}
pub(crate) const CACHE_CLEAN: GpuRegister = GpuRegister(bit(0));
pub(crate) const CACHE_INV: GpuRegister = GpuRegister(bit(1));
pub(crate) const GPU_STATUS: GpuRegister = GpuRegister(0x34);
pub(crate) const GPU_STATUS_ACTIVE: GpuRegister = GpuRegister(bit(0));
pub(crate) const GPU_STATUS_PWR_ACTIVE: GpuRegister = GpuRegister(bit(1));
pub(crate) const GPU_STATUS_PAGE_FAULT: GpuRegister = GpuRegister(bit(4));
pub(crate) const GPU_STATUS_PROTM_ACTIVE: GpuRegister = GpuRegister(bit(7));
pub(crate) const GPU_STATUS_DBG_ENABLED: GpuRegister = GpuRegister(bit(8));
pub(crate) const GPU_FAULT_STATUS: GpuRegister = GpuRegister(0x3c);
pub(crate) const GPU_FAULT_ADDR_LO: GpuRegister = GpuRegister(0x40);
pub(crate) const GPU_FAULT_ADDR_HI: GpuRegister = GpuRegister(0x44);
pub(crate) const GPU_PWR_KEY: GpuRegister = GpuRegister(0x50);
pub(crate) const GPU_PWR_KEY_UNLOCK: GpuRegister = GpuRegister(0x2968a819);
pub(crate) const GPU_PWR_OVERRIDE0: GpuRegister = GpuRegister(0x54);
pub(crate) const GPU_PWR_OVERRIDE1: GpuRegister = GpuRegister(0x58);
pub(crate) const GPU_TIMESTAMP_OFFSET_LO: GpuRegister = GpuRegister(0x88);
pub(crate) const GPU_TIMESTAMP_OFFSET_HI: GpuRegister = GpuRegister(0x8c);
pub(crate) const GPU_CYCLE_COUNT_LO: GpuRegister = GpuRegister(0x90);
pub(crate) const GPU_CYCLE_COUNT_HI: GpuRegister = GpuRegister(0x94);
pub(crate) const GPU_TIMESTAMP_LO: GpuRegister = GpuRegister(0x98);
pub(crate) const GPU_TIMESTAMP_HI: GpuRegister = GpuRegister(0x9c);
pub(crate) const GPU_THREAD_MAX_THREADS: GpuRegister = GpuRegister(0xa0);
pub(crate) const GPU_THREAD_MAX_WORKGROUP_SIZE: GpuRegister = GpuRegister(0xa4);
pub(crate) const GPU_THREAD_MAX_BARRIER_SIZE: GpuRegister = GpuRegister(0xa8);
pub(crate) const GPU_THREAD_FEATURES: GpuRegister = GpuRegister(0xac);
pub(crate) const fn gpu_texture_features(n: u64) -> GpuRegister {
    GpuRegister(0xB0 + ((n) * 4))
}
pub(crate) const GPU_SHADER_PRESENT_LO: GpuRegister = GpuRegister(0x100);
pub(crate) const GPU_SHADER_PRESENT_HI: GpuRegister = GpuRegister(0x104);
pub(crate) const GPU_TILER_PRESENT_LO: GpuRegister = GpuRegister(0x110);
pub(crate) const GPU_TILER_PRESENT_HI: GpuRegister = GpuRegister(0x114);
pub(crate) const GPU_L2_PRESENT_LO: GpuRegister = GpuRegister(0x120);
pub(crate) const GPU_L2_PRESENT_HI: GpuRegister = GpuRegister(0x124);
pub(crate) const SHADER_READY_LO: GpuRegister = GpuRegister(0x140);
pub(crate) const SHADER_READY_HI: GpuRegister = GpuRegister(0x144);
pub(crate) const TILER_READY_LO: GpuRegister = GpuRegister(0x150);
pub(crate) const TILER_READY_HI: GpuRegister = GpuRegister(0x154);
pub(crate) const L2_READY_LO: GpuRegister = GpuRegister(0x160);
pub(crate) const L2_READY_HI: GpuRegister = GpuRegister(0x164);
pub(crate) const SHADER_PWRON_LO: GpuRegister = GpuRegister(0x180);
pub(crate) const SHADER_PWRON_HI: GpuRegister = GpuRegister(0x184);
pub(crate) const TILER_PWRON_LO: GpuRegister = GpuRegister(0x190);
pub(crate) const TILER_PWRON_HI: GpuRegister = GpuRegister(0x194);
pub(crate) const L2_PWRON_LO: GpuRegister = GpuRegister(0x1a0);
pub(crate) const L2_PWRON_HI: GpuRegister = GpuRegister(0x1a4);
pub(crate) const SHADER_PWROFF_LO: GpuRegister = GpuRegister(0x1c0);
pub(crate) const SHADER_PWROFF_HI: GpuRegister = GpuRegister(0x1c4);
pub(crate) const TILER_PWROFF_LO: GpuRegister = GpuRegister(0x1d0);
pub(crate) const TILER_PWROFF_HI: GpuRegister = GpuRegister(0x1d4);
pub(crate) const L2_PWROFF_LO: GpuRegister = GpuRegister(0x1e0);
pub(crate) const L2_PWROFF_HI: GpuRegister = GpuRegister(0x1e4);
pub(crate) const SHADER_PWRTRANS_LO: GpuRegister = GpuRegister(0x200);
pub(crate) const SHADER_PWRTRANS_HI: GpuRegister = GpuRegister(0x204);
pub(crate) const TILER_PWRTRANS_LO: GpuRegister = GpuRegister(0x210);
pub(crate) const TILER_PWRTRANS_HI: GpuRegister = GpuRegister(0x214);
pub(crate) const L2_PWRTRANS_LO: GpuRegister = GpuRegister(0x220);
pub(crate) const L2_PWRTRANS_HI: GpuRegister = GpuRegister(0x224);
pub(crate) const SHADER_PWRACTIVE_LO: GpuRegister = GpuRegister(0x240);
pub(crate) const SHADER_PWRACTIVE_HI: GpuRegister = GpuRegister(0x244);
pub(crate) const TILER_PWRACTIVE_LO: GpuRegister = GpuRegister(0x250);
pub(crate) const TILER_PWRACTIVE_HI: GpuRegister = GpuRegister(0x254);
pub(crate) const L2_PWRACTIVE_LO: GpuRegister = GpuRegister(0x260);
pub(crate) const L2_PWRACTIVE_HI: GpuRegister = GpuRegister(0x264);
pub(crate) const GPU_REVID: GpuRegister = GpuRegister(0x280);
pub(crate) const GPU_COHERENCY_FEATURES: GpuRegister = GpuRegister(0x300);
pub(crate) const GPU_COHERENCY_PROTOCOL: GpuRegister = GpuRegister(0x304);
pub(crate) const GPU_COHERENCY_ACE: GpuRegister = GpuRegister(0);
pub(crate) const GPU_COHERENCY_ACE_LITE: GpuRegister = GpuRegister(1);
pub(crate) const GPU_COHERENCY_NONE: GpuRegister = GpuRegister(31);
pub(crate) const MCU_CONTROL: GpuRegister = GpuRegister(0x700);
pub(crate) const MCU_CONTROL_ENABLE: GpuRegister = GpuRegister(1);
pub(crate) const MCU_CONTROL_AUTO: GpuRegister = GpuRegister(2);
pub(crate) const MCU_CONTROL_DISABLE: GpuRegister = GpuRegister(0);
pub(crate) const MCU_STATUS: GpuRegister = GpuRegister(0x704);
pub(crate) const MCU_STATUS_DISABLED: GpuRegister = GpuRegister(0);
pub(crate) const MCU_STATUS_ENABLED: GpuRegister = GpuRegister(1);
pub(crate) const MCU_STATUS_HALT: GpuRegister = GpuRegister(2);
pub(crate) const MCU_STATUS_FATAL: GpuRegister = GpuRegister(3);
pub(crate) const JOB_INT_RAWSTAT: GpuRegister = GpuRegister(0x1000);
pub(crate) const JOB_INT_CLEAR: GpuRegister = GpuRegister(0x1004);
pub(crate) const JOB_INT_MASK: GpuRegister = GpuRegister(0x1008);
pub(crate) const JOB_INT_STAT: GpuRegister = GpuRegister(0x100c);
pub(crate) const JOB_INT_GLOBAL_IF: GpuRegister = GpuRegister(bit(31));
pub(crate) const fn job_int_csg_if(x: u64) -> GpuRegister {
    GpuRegister(bit(x))
}
pub(crate) const MMU_INT_RAWSTAT: GpuRegister = GpuRegister(0x2000);
pub(crate) const MMU_INT_CLEAR: GpuRegister = GpuRegister(0x2004);
pub(crate) const MMU_INT_MASK: GpuRegister = GpuRegister(0x2008);
pub(crate) const MMU_INT_STAT: GpuRegister = GpuRegister(0x200c);
pub(crate) const MMU_BASE: GpuRegister = GpuRegister(0x2400);
pub(crate) const MMU_AS_SHIFT: GpuRegister = GpuRegister(6);
const fn mmu_as(as_: u64) -> u64 {
    MMU_BASE.0 + ((as_) << MMU_AS_SHIFT.0)
}
pub(crate) const fn as_transtab_lo(as_: u64) -> GpuRegister {
    GpuRegister(mmu_as(as_) + 0x0)
}
pub(crate) const fn as_transtab_hi(as_: u64) -> GpuRegister {
    GpuRegister(mmu_as(as_) + 0x4)
}
pub(crate) const fn as_memattr_lo(as_: u64) -> GpuRegister {
    GpuRegister(mmu_as(as_) + 0x8)
}
pub(crate) const fn as_memattr_hi(as_: u64) -> GpuRegister {
    GpuRegister(mmu_as(as_) + 0xC)
}
pub(crate) const fn as_memattr_aarch64_inner_alloc_expl(w: u64, r: u64) -> GpuRegister {
    GpuRegister((3 << 2) | (if w > 0 { bit(0) } else { 0 } | (if r > 0 { bit(1) } else { 0 })))
}
pub(crate) const fn as_lockaddr_lo(as_: u64) -> GpuRegister {
    GpuRegister(mmu_as(as_) + 0x10)
}
pub(crate) const fn as_lockaddr_hi(as_: u64) -> GpuRegister {
    GpuRegister(mmu_as(as_) + 0x14)
}
pub(crate) const fn as_command(as_: u64) -> GpuRegister {
    GpuRegister(mmu_as(as_) + 0x18)
}
pub(crate) const AS_COMMAND_NOP: GpuRegister = GpuRegister(0);
pub(crate) const AS_COMMAND_UPDATE: GpuRegister = GpuRegister(1);
pub(crate) const AS_COMMAND_LOCK: GpuRegister = GpuRegister(2);
pub(crate) const AS_COMMAND_UNLOCK: GpuRegister = GpuRegister(3);
pub(crate) const AS_COMMAND_FLUSH_PT: GpuRegister = GpuRegister(4);
pub(crate) const AS_COMMAND_FLUSH_MEM: GpuRegister = GpuRegister(5);
pub(crate) const fn as_faultstatus(as_: u64) -> GpuRegister {
    GpuRegister(mmu_as(as_) + 0x1C)
}
pub(crate) const fn as_faultaddress_lo(as_: u64) -> GpuRegister {
    GpuRegister(mmu_as(as_) + 0x20)
}
pub(crate) const fn as_faultaddress_hi(as_: u64) -> GpuRegister {
    GpuRegister(mmu_as(as_) + 0x24)
}
pub(crate) const fn as_status(as_: u64) -> GpuRegister {
    GpuRegister(mmu_as(as_) + 0x28)
}
pub(crate) const AS_STATUS_AS_ACTIVE: GpuRegister = GpuRegister(bit(0));
pub(crate) const fn as_transcfg_lo(as_: u64) -> GpuRegister {
    GpuRegister(mmu_as(as_) + 0x30)
}
pub(crate) const fn as_transcfg_hi(as_: u64) -> GpuRegister {
    GpuRegister(mmu_as(as_) + 0x34)
}
pub(crate) const fn as_transcfg_ina_bits(x: u64) -> GpuRegister {
    GpuRegister((x) << 6)
}
pub(crate) const fn as_transcfg_outa_bits(x: u64) -> GpuRegister {
    GpuRegister((x) << 14)
}
pub(crate) const AS_TRANSCFG_SL_CONCAT: GpuRegister = GpuRegister(bit(22));
pub(crate) const AS_TRANSCFG_PTW_RA: GpuRegister = GpuRegister(bit(30));
pub(crate) const AS_TRANSCFG_DISABLE_HIER_AP: GpuRegister = GpuRegister(bit(33));
pub(crate) const AS_TRANSCFG_DISABLE_AF_FAULT: GpuRegister = GpuRegister(bit(34));
pub(crate) const AS_TRANSCFG_WXN: GpuRegister = GpuRegister(bit(35));
pub(crate) const AS_TRANSCFG_XREADABLE: GpuRegister = GpuRegister(bit(36));
pub(crate) const fn as_faultextra_lo(as_: u64) -> GpuRegister {
    GpuRegister(mmu_as(as_) + 0x38)
}
pub(crate) const fn as_faultextra_hi(as_: u64) -> GpuRegister {
    GpuRegister(mmu_as(as_) + 0x3C)
}
pub(crate) const CSF_GPU_LATEST_FLUSH_ID: GpuRegister = GpuRegister(0x10000);
pub(crate) const fn csf_doorbell(i: u64) -> GpuRegister {
    GpuRegister(0x80000 + ((i) * 0x10000))
}
pub(crate) const CSF_GLB_DOORBELL_ID: GpuRegister = GpuRegister(0);
