use register::cpu::RegisterReadWrite;
register_bitfields! {u64,
  pub SATP [
    MODE OFFSET(60) NUMBITS(4) [
      Bare = 0,
      Sv39 = 8,
      Sv48 = 9
    ],
    ASID OFFSET(44) NUMBITS(16) [],
    PPN OFFSET(0) NUMBITS(44) []
  ]
}
pub struct Reg;

impl RegisterReadWrite<u64, SATP::Register> for Reg {
  sys_csr_read_raw!(u64, "SATP");
  sys_csr_write_raw!(u64, "SATP");
}

pub static SATP: Reg = Reg {};