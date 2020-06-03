// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Jorge Aparicio
//   - Andre Richter <andre.o.richter@gmail.com>

//! Miscellaneous assembly instructions

use core;

/// The classic no-op
#[inline(always)]
pub fn nop() {
  match () {
    #[cfg(target_arch = "riscv64")]
    () => unsafe { llvm_asm!("nop" :::: "volatile") },

    #[cfg(not(target_arch = "riscv64"))]
    () => unimplemented!(),
  }
}

/// Wait For Interrupt
#[inline(always)]
pub fn wfi() {
  match () {
    #[cfg(target_arch = "riscv64")]
    () => unsafe { llvm_asm!("wfi" :::: "volatile") },

    #[cfg(not(target_arch = "riscv64"))]
    () => unimplemented!(),
  }
}

/// Exception return
///
/// Will jump to wherever the corresponding link register points to, and therefore never return.
#[inline(always)]
pub fn eret() -> ! {
  match () {
    #[cfg(target_arch = "riscv64")]
    () => unsafe {
      llvm_asm!("eret" :::: "volatile");
      core::intrinsics::unreachable()
    },

    #[cfg(not(target_arch = "riscv64"))]
    () => unimplemented!(),
  }
}

/// Function return
///
/// Will jump to wherever the corresponding link register points to, and therefore never return.
#[inline(always)]
pub fn ret() -> ! {
  match () {
    #[cfg(target_arch = "riscv64")]
    () => unsafe {
      llvm_asm!("ret" :::: "volatile");
      core::intrinsics::unreachable()
    },

    #[cfg(not(target_arch = "riscv64"))]
    () => unimplemented!(),
  }
}
