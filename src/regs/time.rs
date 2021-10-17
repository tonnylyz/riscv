use tock_registers::interfaces::Readable;

pub struct Reg;

impl Readable for Reg {
  type T = u64;
  type R = ();
  sys_csr_read_raw!(u64, "TIME");
}

pub static TIME: Reg = Reg {};