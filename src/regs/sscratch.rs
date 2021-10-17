use tock_registers::interfaces::{Readable, Writeable};

pub struct Reg;

impl Readable for Reg {
  type T = u64;
  type R = ();
  sys_csr_read_raw!(u64, "SSCRATCH");
}

impl Writeable for Reg {
  type T = u64;
  type R = ();
  sys_csr_write_raw!(u64, "SSCRATCH");
}

pub static SSCRATCH: Reg = Reg {};