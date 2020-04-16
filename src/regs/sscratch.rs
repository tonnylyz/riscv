use register::cpu::RegisterReadWrite;

pub struct Reg;

impl RegisterReadWrite<u64, ()> for Reg {
  sys_csr_read_raw!(u64, "SSCRATCH");
  sys_csr_write_raw!(u64, "SSCRATCH");
}

pub static SSCRATCH: Reg = Reg {};