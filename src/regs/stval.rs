use register::cpu::RegisterReadWrite;

pub struct Reg;

impl RegisterReadWrite<u64, ()> for Reg {
  sys_csr_read_raw!(u64, "STVAL");
  sys_csr_write_raw!(u64, "STVAL");
}

pub static STVAL: Reg = Reg {};