use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields
  };
  
  register_bitfields! {u64,
    pub FCSR [
      FRM OFFSET(5) NUMBITS(3) [],
      NV OFFSET(4) NUMBITS(1) [],
      DZ OFFSET(3) NUMBITS(1) [],
      OF OFFSET(2) NUMBITS(1) [],
      UF OFFSET(1) NUMBITS(1) [],
      NX OFFSET(0) NUMBITS(1) []
    ]
  }
  
  pub struct Reg;
  
  impl Readable for Reg {
    type T = u32;
    type R = FCSR::Register;
    sys_csr_read_raw!(u32, "FCSR");
  }
  
  impl Writeable for Reg {
    type T = u32;
    type R = FCSR::Register;
    sys_csr_write_raw!(u32, "FCSR");
  }
  
  pub static FCSR: Reg = Reg {};
