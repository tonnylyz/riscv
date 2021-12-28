
#[inline(always)]
pub fn sfence_vma_all() {
  match () {
    #[cfg(target_arch = "riscv64")]
    () => unsafe {
      core::arch::asm!("sfence.vma")
    },

    #[cfg(not(target_arch = "riscv64"))]
    () => unimplemented!(),
  }
}

#[inline(always)]
pub fn sfence_vma(asid: usize, addr: usize) {
  match () {
    #[cfg(target_arch = "riscv64")]
    () => unsafe {
      core::arch::asm!("sfence.vma {0}, {1}", in(reg) asid, in(reg) addr)
    },

    #[cfg(not(target_arch = "riscv64"))]
    () => unimplemented!(),
  }
}