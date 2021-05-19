#![no_std]
#![feature(asm)]
#![feature(custom_inner_attributes)]
#![feature(core_intrinsics)]

#[macro_use]
extern crate register;

pub mod asm;
pub mod barrier;
pub mod regs;