// SPDX-License-Identifier: GPL-2.0
// SPDX-FileCopyrightText: Copyright Collabora 2024

//! The Rust components of the Panthor driver

// Just while developing
#![allow(unused_imports,dead_code, unused_variables)]

#[cfg(CONFIG_DRM_PANTHOR_COREDUMP)]
mod dump;
mod regs;

const __LOG_PREFIX: &[u8] = b"panthor\0";
