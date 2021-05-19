// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Jorge Aparicio
//   - Andre Richter <andre.o.richter@gmail.com>

//! Miscellaneous assembly instructions

/// The classic no-op
#[inline(always)]
pub fn nop() {
  match () {
    #[cfg(target_arch = "riscv64")]
    () => unsafe { asm!("nop") },

    #[cfg(not(target_arch = "riscv64"))]
    () => unimplemented!(),
  }
}

/// Wait For Interrupt
#[inline(always)]
pub fn wfi() {
  match () {
    #[cfg(target_arch = "riscv64")]
    () => unsafe { asm!("wfi") },

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
      asm!("eret", options(noreturn))
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
      asm!("ret", options(noreturn))
    },

    #[cfg(not(target_arch = "riscv64"))]
    () => unimplemented!(),
  }
}
