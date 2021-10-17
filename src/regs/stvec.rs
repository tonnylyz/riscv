use tock_registers::{
  interfaces::{Readable, Writeable},
  register_bitfields
};

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

impl Readable for Reg {
  type T = u64;
  type R = STVEC::Register;
  sys_csr_read_raw!(u64, "STVEC");
}

impl Writeable for Reg {
  type T = u64;
  type R = STVEC::Register;
  sys_csr_write_raw!(u64, "STVEC");
}

pub static STVEC: Reg = Reg {};