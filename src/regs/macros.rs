// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>
//   - tonnylyz <lyztonny@gmail.com>

/// Raw read from system control registers.
macro_rules! sys_csr_read_raw {
   ($width:ty, $csr_name:tt) => {
        /// Reads the raw bits of the CPU register.
        #[inline]
        fn get(&self) -> $width {
            match () {
                #[cfg(target_arch = "riscv64")]
                () => {
                    let reg;
                    unsafe {
                        asm!(concat!("csrrs {}, ", $csr_name, ", x0"), out(reg) reg, options(nomem, nostack));
                    }
                    reg
                }

                #[cfg(not(target_arch = "riscv64"))]
                () => unimplemented!(),
            }
        }
    };
}

/// Raw write to system control registers.
macro_rules! sys_csr_write_raw {
    ($width:ty, $csr_name:tt) => {
        /// Writes raw bits to the CPU register.
        #[allow(unused_variables)]
        #[inline]
        fn set(&self, value: $width) {
            match () {
                #[cfg(target_arch = "riscv64")]
                () => {
                    unsafe {
                        asm!(concat!("csrrw x0, ", $csr_name, ", {}"), in(reg) value, options(nomem, nostack))
                    }
                }

                #[cfg(not(target_arch = "riscv64"))]
                () => unimplemented!(),
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! sys_csr_set_raw {
    ($width:ty, $csr_name:tt) => {
        /// Set the CSR
        #[allow(unused_variables)]
        #[inline]
        fn set(&self, value: $width) {
            match () {
                #[cfg(target_arch = "riscv64")]
                () => unsafe {
                    asm!(concat!("csrrs x0, ", $csr_name, ", {}"), in(reg) value, options(nomem, nostack))
                },

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
        fn set(&self, value: $width) {
            match () {
                #[cfg(target_arch = "riscv64")]
                () => unsafe {
                    asm!(concat!("csrrc x0, ", $csr_name, ", {}"), in(reg) value, options(nomem, nostack))
                },

                #[cfg(not(target_arch = "riscv64"))]
                () => unimplemented!(),
            }
        }
    };
}
