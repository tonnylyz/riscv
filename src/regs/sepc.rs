use register::cpu::RegisterReadWrite;

pub struct Reg;

impl RegisterReadWrite<u64, ()> for Reg {
  sys_csr_read_raw!(u64, "SEPC");
  sys_csr_write_raw!(u64, "SEPC");
}

pub static SEPC: Reg = Reg {};