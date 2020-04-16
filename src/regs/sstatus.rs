use register::cpu::RegisterReadWrite;

register_bitfields! {u64,
  pub SSTATUS [
    SD OFFSET(63) NUMBITS(1) [],
    UXL OFFSET(32) NUMBITS(2) [],
    MXR OFFSET(19) NUMBITS(1) [],
    SUM OFFSET(18) NUMBITS(1) [],
    XS OFFSET(15) NUMBITS(2) [],
    FS OFFSET(13) NUMBITS(2) [],
    SPP OFFSET(8) NUMBITS(1) [
      User = 0,
      Supervisor = 1
    ],
    SPIE OFFSET(5) NUMBITS(1) [],
    UPIE OFFSET(4) NUMBITS(1) [],
    SIE OFFSET(1) NUMBITS(1) [],
    UIE OFFSET(0) NUMBITS(1) []
  ]
}

pub struct Reg;

impl RegisterReadWrite<u64, SSTATUS::Register> for Reg {
  sys_csr_read_raw!(u64, "SSTATUS");
  sys_csr_write_raw!(u64, "SSTATUS");
}

pub static SSTATUS: Reg = Reg {};