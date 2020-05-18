use register::cpu::RegisterReadOnly;

pub struct Reg;

impl RegisterReadOnly<u64, ()> for Reg {
  sys_csr_read_raw!(u64, "MHARTID");
}

pub static MHARTID: Reg = Reg {};