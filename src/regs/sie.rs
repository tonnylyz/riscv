use tock_registers::{
  interfaces::{Readable, Writeable},
  register_bitfields,
};

// sie is the corresponding SXLEN-bit read/write register containing interrupt enable bits
register_bitfields! {u64,
  pub SIE [
    SEIE OFFSET(9) NUMBITS(1) [],
    UEIE OFFSET(8) NUMBITS(1) [],
    STIE OFFSET(5) NUMBITS(1) [],
    UTIE OFFSET(4) NUMBITS(1) [],
    SSIE OFFSET(1) NUMBITS(1) [],
    USIE OFFSET(0) NUMBITS(1) []
  ]
}

pub struct Reg;

impl Readable for Reg {
  type T = u64;
  type R = SIE::Register;
  sys_csr_read_raw!(u64, "SIE");
}

impl Writeable for Reg {
  type T = u64;
  type R = SIE::Register;
  sys_csr_write_raw!(u64, "SIE");
}

pub static SIE: Reg = Reg {};