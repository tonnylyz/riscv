// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>
//   - tonnylyz <lyztonny@gmail.com>

macro_rules! __read_raw {
    ($width:ty, $asm_instr:tt, $csr_name:tt) => {
        /// Reads the raw bits of the CPU register.
        #[inline]
        fn get(&self) -> $width {
            match () {
                #[cfg(target_arch = "riscv64")]
                () => {
                    let reg;
                    unsafe {
                        asm!(concat!($asm_instr, " $0, ", $csr_name, ", x10") : "=r"(reg) ::: "volatile");
                    }
                    reg
                }

                #[cfg(not(target_arch = "riscv64"))]
                () => unimplemented!(),
            }
        }
    };
}

macro_rules! __write_raw {
    ($width:ty, $asm_instr:tt, $csr_name:tt) => {
        /// Writes raw bits to the CPU register.
        #[cfg_attr(not(target_arch = "riscv64"), allow(unused_variables))]
        #[inline]
        fn set(&self, value: $width) {
            match () {
                #[cfg(target_arch = "riscv64")]
                () => {
                    unsafe {
                        asm!(concat!($asm_instr, " x0, ", $csr_name, ", $0") :: "r"(value) :: "volatile")
                    }
                }

                #[cfg(not(target_arch = "riscv64"))]
                () => unimplemented!(),
            }
        }
    };
}

/// Raw read from system control registers.
macro_rules! sys_csr_read_raw {
    ($width:ty, $csr_name:tt) => {
        __read_raw!($width, "csrrs", $csr_name);
    };
}

/// Raw write to system control registers.
macro_rules! sys_csr_write_raw {
    ($width:ty, $csr_name:tt) => {
        __write_raw!($width, "csrrw", $csr_name);
    };
}

/// Raw read from (ordinary) registers.
#[allow(unused_macros)]
macro_rules! read_raw {
    ($width:ty, $csr_name:tt) => {
        __read_raw!($width, "mv", $csr_name);
    };
}

/// Raw write to (ordinary) registers.
#[allow(unused_macros)]
macro_rules! write_raw {
    ($width:ty, $csr_name:tt) => {
        __write_raw!($width, "mv", $csr_name);
    };
}

#[allow(unused_macros)]
macro_rules! sys_csr_set_raw {
    ($width:ty, $csr_name:tt) => {
        /// Set the CSR
        #[inline]
        fn set_bits(bits: $width) {
            match () {
                #[cfg(target_arch = "riscv64")]
                () => asm!("csrrs x0, $1, $0" :: "r"(bits), "i"($csr_name) :: "volatile"),

                #[cfg(not(target_arch = "riscv64"))]
                () => unimplemented!(),
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! sys_csr_clear_raw {
    ($width:ty, $csr_name:tt) => {
        /// Clear the CSR
        #[inline]
        #[allow(unused_variables)]
        fn clear_bits(bits: $width) {
            match () {
                #[cfg(target_arch = "riscv64")]
                () => asm!("csrrc x0, $1, $0" :: "r"(bits), "i"($csr_name) :: "volatile"),

                #[cfg(not(target_arch = "riscv64"))]
                () => unimplemented!(),
            }
        }
    };
}
