use register::cpu::RegisterReadWrite;

register_bitfields! {u64,
  pub STVEC [
    BASE OFFSET(2) NUMBITS(62) [],
    MODE OFFSET(0) NUMBITS(2) [
      Direct = 0,
      Vectored = 1
    ]
  ]
}

pub struct Reg;

impl RegisterReadWrite<u64, STVEC::Register> for Reg {
  sys_csr_read_raw!(u64, "STVEC");
  sys_csr_write_raw!(u64, "STVEC");
}

pub static STVEC: Reg = Reg {};