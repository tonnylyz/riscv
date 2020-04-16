
#[inline(always)]
pub fn sfence_vma_all() {
  match () {
    #[cfg(target_arch = "riscv64")]
    () => unsafe {
      asm!("sfence.vma" :::: "volatile")
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
      asm!("sfence.vma $0, $1" :: "r"(asid), "r"(addr) :: "volatile")
    },

    #[cfg(not(target_arch = "riscv64"))]
    () => unimplemented!(),
  }
}