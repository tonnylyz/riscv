use tock_registers::{
  interfaces::Readable,
  register_bitfields
};

register_bitfields! {u64,
  pub SCAUSE [
    INTERRUPT OFFSET(63) NUMBITS(1) [
      True = 1,
      False = 0
    ],
  /*Interrupt Exception Code  Description
        1             0       User software interrupt
        1             1       Supervisor software interrupt
        1             2       Reserved for future standard use
        1             3       Machine software interrupt
        1             4       User timer interrupt
        1             5       Supervisor timer interrupt
        1             6       Reserved for future standard use
        1             7       Machine timer interrupt
        1             8       User external interrupt
        1             9       Supervisor external interrupt
        1             10      Reserved for future standard use
        1             11      Machine external interrupt
        1             12–15   Reserved for future standard use
        1             ≥16     Reserved for platform use
        0             0       Instruction address misaligned
        0             1       Instruction access fault
        0             2       Illegal instruction
        0             3       Breakpoint
        0             4       Load address misaligned
        0             5       Load access fault
        0             6       Store/AMO address misaligned
        0             7       Store/AMO access fault
        0             8       Environment call from U-mode
        0             9       Environment call from S-mode
        0             10      Reserved
        0             11      Environment call from M-mode
        0             12      Instruction page fault
        0             13      Load page fault
        0             14      Reserved for future standard use
        0             15      Store/AMO page fault
        0             16–23   Reserved for future standard use
        0             24–31   Reserved for custom use
        0             32–47   Reserved for future standard use
        0             48–63   Reserved for custom use
        0             ≥64     Reserved for future standard use
    */
    CODE OFFSET(0) NUMBITS(8) []
  ]
}

pub struct Reg;

impl Readable for Reg {
  type T = u64;
  type R = SCAUSE::Register;
  sys_csr_read_raw!(u64, "SCAUSE");
}

pub static SCAUSE: Reg = Reg {};