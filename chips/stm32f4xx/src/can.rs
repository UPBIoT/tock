use core::cell::Cell;
use cortex_m_semihosting::hprintln;
use kernel::common::StaticRef;
use kernel::common::registers::{register_bitfields, register_structs, ReadWrite};
use kernel::ClockInterface;
use kernel::debug;

use crate::rcc;
register_structs! {
    CanRegisters {
        /// CAN control and status registers
        (0x000 => can_mcr: ReadWrite<u32, CAN_MCR::Register>),
        /// CAN master status register
        (0x004 => can_msr: ReadWrite<u32, CAN_MSR::Register>),
        /// CAN transmit status register
        (0x008 => can_tsr: ReadWrite<u32, CAN_TSR::Register>),
        /// CAN receive FIFO 0 register
        (0x00c => can_rf0r: ReadWrite<u32, CAN_RF0R::Register>),
        /// CAN receive FIFO 1 register
        (0x010 => can_rf1r: ReadWrite<u32, CAN_RF1R::Register>),
        /// CAN interrupt enable register
        (0x014 => can_ier: ReadWrite<u32, CAN_IER::Register>),
        /// CAN error status register
        (0x018 => can_esr: ReadWrite<u32, CAN_ESR::Register>),
        /// CAN bit timing register
        (0x01c => can_btr: ReadWrite<u32, CAN_BTR::Register>),
        (0x020 => _reserved0),
        ///
        /// 
        /// CAN MAILBOX REGISTERS
        /// 
        /// 
        /// CAN TX mailbox identifier register 0
        (0x180 => can_ti0r: ReadWrite<u32, CAN_TIxR::Register>),
        /// CAN mailbox data length control and time stamp register 0
        (0x184 => can_tdt0r: ReadWrite<u32, CAN_TDTxR::Register>),
        /// CAN mailbox data low register 0
        (0x188 => can_tdl0r: ReadWrite<u32, CAN_TDLxR::Register>),
        /// CAN mailbox data high register 0
        (0x18c => can_tdh0r: ReadWrite<u32, CAN_TDHxR::Register>),
        /// CAN TX mailbox identifier register 1
        (0x190 => can_ti1r: ReadWrite<u32, CAN_TIxR::Register>),
        /// CAN mailbox data length control and time stamp register 1
        (0x194 => can_tdt1r: ReadWrite<u32, CAN_TDTxR::Register>),
        /// CAN mailbox data low register 1
        (0x198 => can_tdl1r: ReadWrite<u32, CAN_TDLxR::Register>),
        /// CAN mailbox data high register 1
        (0x19c => can_tdh1r: ReadWrite<u32, CAN_TDHxR::Register>),
        /// CAN TX mailbox identifier register 2
        (0x1a0 => can_ti2r: ReadWrite<u32, CAN_TIxR::Register>),
        /// CAN mailbox data length control and time stamp register 2
        (0x1a4 => can_tdt2r: ReadWrite<u32, CAN_TDTxR::Register>),
        /// CAN mailbox data low register 2
        (0x1a8 => can_tdl2r: ReadWrite<u32, CAN_TDLxR::Register>),
        /// CAN mailbox data high register 2
        (0x1ac => can_tdh2r: ReadWrite<u32, CAN_TDHxR::Register>),
        /// CAN receive FIFO mailbox identifier register 0
        (0x1b0 => can_ri0r: ReadWrite<u32, CAN_RIxR::Register>),
        /// CAN receive FIFO mailbox data length control and timpe stamp register 0
        (0x1b4 => can_rdt0r: ReadWrite<u32, CAN_RDTxR::Register>),
        /// CAN receive FIFO mailbox data low register 0
        (0x1b8 => can_rdl0r: ReadWrite<u32, CAN_RDLxR::Register>),
        /// CAN receive FIFO mailbox data high register 0
        (0x1bc => can_rdh0r: ReadWrite<u32, CAN_RDHxR::Register>),
        /// CAN receive FIFO mailbox identifier register 1
        (0x1c0 => can_ri1r: ReadWrite<u32, CAN_RIxR::Register>),
        /// CAN receive FIFO mailbox data length control and timpe stamp register 1
        (0x1c4 => can_rdt1r: ReadWrite<u32, CAN_RDTxR::Register>),
        /// CAN receive FIFO mailbox data low register 1
        (0x1c8 => can_rdl1r: ReadWrite<u32, CAN_RDLxR::Register>),
        /// CAN receive FIFO mailbox data high register 1
        (0x1cc => can_rdh1r: ReadWrite<u32, CAN_RDHxR::Register>),
        (0x1d0 => _reserved1),
        /// 
        /// 
        /// CAN FILTER REGISTERS
        /// 
        ///
        /// CAN filter master register
        (0x200 => can_fmr: ReadWrite<u32, CAN_FMR::Register>),
        /// CAN filter mode register
        (0x204 => can_fm1r: ReadWrite<u32, CAN_FM1R::Register>),
        (0x208 => _reserved2),
        /// CAN filter scale register
        (0x20c => can_fs1r: ReadWrite<u32, CAN_FS1R::Register>),
        (0x210 => _reserved3),
        /// CAN filter FIFO assignment register
        (0x214 => can_ffa1r: ReadWrite<u32, CAN_FFA1R::Register>),
        (0x218 => _reserved4),
        /// CAN filter activation register
        (0x21c => can_fa1r: ReadWrite<u32, CAN_FA1R::Register>),
        (0x220 => _reserved5),
        /// Filter bank 0 for register 1
        (0x240 => can_f0r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 0 for register 2
        (0x244 => can_f0r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 1 for register 1
        (0x248 => can_f1r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 1 for register 2
        (0x24c => can_f1r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 2 for register 1
        (0x250 => can_f2r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 2 for register 2
        (0x254 => can_f2r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 3 for register 1
        (0x258 => can_f3r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 3 for register 2
        (0x25c => can_f3r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 4 for register 1
        (0x260 => can_f4r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 4 for register 2
        (0x264 => can_f4r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 5 for register 1
        (0x268 => can_f5r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 5 for register 2
        (0x26c => can_f5r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 6 for register 1
        (0x270 => can_f6r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 6 for register 2
        (0x274 => can_f6r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 7 for register 1
        (0x278 => can_f7r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 7 for register 2
        (0x27c => can_f7r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 8 for register 1
        (0x280 => can_f8r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 8 for register 2
        (0x284 => can_f8r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 9 for register 1
        (0x288 => can_f9r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 9 for register 2
        (0x28c => can_f9r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 10 for register 1
        (0x290 => can_f10r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 10 for register 2
        (0x294 => can_f10r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 11 for register 1
        (0x298 => can_f11r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 11 for register 2
        (0x29c => can_f11r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 12 for register 1
        (0x2a0 => can_f12r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 12 for register 2
        (0x2a4 => can_f12r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 13 for register 1
        (0x2a8 => can_f13r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 13 for register 2
        (0x2ac => can_f13r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 14 for register 1
        (0x2b0 => can_f14r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 14 for register 2
        (0x2b4 => can_f14r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 15 for register 1
        (0x2b8 => can_f15r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 15 for register 2
        (0x2bc => can_f15r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 16 for register 1
        (0x2c0 => can_f16r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 16 for register 2
        (0x2c4 => can_f16r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 17 for register 1
        (0x2c8 => can_f17r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 17 for register 2
        (0x2cc => can_f17r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 18 for register 1
        (0x2d0 => can_f18r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 18 for register 2
        (0x2d4 => can_f18r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 19 for register 1
        (0x2d8 => can_f19r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 19 for register 2
        (0x2dc => can_f19r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 20 for register 1
        (0x2e0 => can_f20r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 20 for register 2
        (0x2e4 => can_f20r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 21 for register 1
        (0x2e8 => can_f21r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 21 for register 2
        (0x2ec => can_f21r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 22 for register 1
        (0x2f0 => can_f22r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 22 for register 2
        (0x2f4 => can_f22r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 23 for register 1
        (0x2f8 => can_f23r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 23 for register 2
        (0x2fc => can_f23r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 24 for register 1
        (0x300 => can_f24r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 24 for register 2
        (0x304 => can_f24r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 25 for register 1
        (0x308 => can_f25r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 25 for register 2
        (0x30c => can_f25r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 26 for register 1
        (0x310 => can_f26r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 26 for register 2
        (0x314 => can_f26r2: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 27 for register 1
        (0x318 => can_f27r1: ReadWrite<u32, CAN_FiRx::Register>),
        /// Filter bank 27 for register 2
        (0x31c => can_f27r2: ReadWrite<u32, CAN_FiRx::Register>),
        (0x320 => @END),
    }
}

register_bitfields![u32,
    CAN_MCR [
        /// Debug freeze
        DBF OFFSET(16) NUMBITS(1) [],
        /// bcXAN software master reset
        RESET OFFSET(15) NUMBITS(1) [],
        /// Time triggered communication mode
        TTCM OFFSET(7) NUMBITS(1) [],
        /// Automatic bus-off management
        ABOM OFFSET(6) NUMBITS(1) [],
        /// Automatic wakeup mode
        AWUM OFFSET(5) NUMBITS(1) [],
        /// No automatic retransmission
        NART OFFSET(4) NUMBITS(1) [],
        /// Receive FIFO locked mode
        RFLM OFFSET(3) NUMBITS(1) [],
        /// Transmit FIFO prioritY
        TXFP OFFSET(2) NUMBITS(1) [],
        /// Sleep mode request
        SLEEP OFFSET(1) NUMBITS(1) [],
        /// Initialization request
        INRQ OFFSET(0) NUMBITS(1) []
    ],
    CAN_MSR [
        /// CAN Rx signal
        RX OFFSET(11) NUMBITS(1) [],
        /// Last sample point
        SAMP OFFSET(10) NUMBITS(1) [],
        /// Receive mode
        RXM OFFSET(9) NUMBITS(1) [],
        /// Transmit mode
        TXM OFFSET(8) NUMBITS(1) [],
        /// Sleep acknowledge interrupt
        SLAKI OFFSET(4) NUMBITS(1) [],
        /// Wakeup interrupt
        WKUI OFFSET(3) NUMBITS(1) [],
        /// Error interrupt
        ERRI OFFSET(2) NUMBITS(1) [],
        /// Sleep acknowledge
        SLAK OFFSET(1) NUMBITS(1) [],
        /// Initialization acknowledge
        INAK OFFSET(0) NUMBITS(1) []
    ],
    CAN_TSR [
        /// Lowest priority flag for mailbox 2
        LOW2 OFFSET(31) NUMBITS(1) [],
        /// Lowest priority flag for mailbox 1
        LOW1 OFFSET(30) NUMBITS(1) [],
        /// Lowest priority flag for mailbox 0
        LOW0 OFFSET(29) NUMBITS(1) [],
        /// Transmit mailbox 2 empty
        TME2 OFFSET(28) NUMBITS(1) [],
        /// Transmit mailbox 1 empty
        TME1 OFFSET(27) NUMBITS(1) [], 
        /// Transmit mailbox 0 empty
        TME0 OFFSET(26) NUMBITS(1) [],
        /// Mailbox code
        CODE OFFSET(24) NUMBITS(2) [],
        /// Abort request for mailbox 2
        ABRQ2 OFFSET(23) NUMBITS(1) [],
        /// Transmission error of mailbox 2
        TERR2 OFFSET(19) NUMBITS(1) [],
        /// Arbitration lost for mailbox 2
        ALST2 OFFSET(18) NUMBITS(1) [],
        /// Transmission OK of mailbox 2
        TXOK2 OFFSET(17) NUMBITS(1) [],
        /// Request completed mailbox 2
        RQCP2 OFFSET(16) NUMBITS(1) [],
        /// Abort request for mailbox 1
        ABRQ1 OFFSET(15) NUMBITS(1) [],
        /// Transmission error of mailbox 1
        TERR1 OFFSET(11) NUMBITS(1) [],
        /// Arbitration lost for mailbox 1
        ALST1 OFFSET(10) NUMBITS(1) [],
        /// Transmission OK of mailbox 1
        TXOK1 OFFSET(9) NUMBITS(1) [],
        /// Request completed mailbox 1
        RQCP1 OFFSET(8) NUMBITS(1) [],
        /// Abort request for mailbox 0
        ABRQ0 OFFSET(7) NUMBITS(1) [],
        /// Transmission error of mailbox 0
        TERR0 OFFSET(3) NUMBITS(1) [],
        /// Arbitration lost for mailbox 0
        ALST0 OFFSET(2) NUMBITS(1) [],
        /// Transmission OK of mailbox 0
        TXOK0 OFFSET(1) NUMBITS(1) [],
        /// Request completed mailbox 0
        RQCP0 OFFSET(0) NUMBITS(1) []
    ],
    CAN_RF0R [
        /// Release FIFO 0 output mailbox
        RFOM0 OFFSET(5) NUMBITS(1) [],
        /// FIFO 0 overrun
        FOVR0 OFFSET(4) NUMBITS(1) [],
        /// FIFO 0 full
        FULL0 OFFSET(3) NUMBITS(1) [],
        /// FIFO 0 message pending
        FMP0 OFFSET(0) NUMBITS(2) []
    ],
    CAN_RF1R [
        /// Release FIFO 1 output mailbox
        RFOM1 OFFSET(5) NUMBITS(1) [],
        /// FIFO 1 overrun
        FOVR1 OFFSET(4) NUMBITS(1) [],
        /// FIFO 1 full
        FULL1 OFFSET(3) NUMBITS(1) [],
        /// FIFO 1 message pending
        FMP1 OFFSET(0) NUMBITS(2) []
    ],
    CAN_IER [
        /// Sleep interrupt enable
        SLKIE OFFSET(17) NUMBITS(1) [],
        /// Wakeup interrupt enable
        WKUIE OFFSET(16) NUMBITS(1) [],
        /// Error interrupt enable
        ERRIE OFFSET(15) NUMBITS(1) [],
        /// Last error code interrupt enable
        LECIE OFFSET(11) NUMBITS(1) [],
        /// Bus-off interrupt enable
        BOFIE OFFSET(10) NUMBITS(1) [],
        /// Error passive interrupt enable
        EPVIE OFFSET(9) NUMBITS(1) [],
        /// Error warning interrupt enable
        EWGIE OFFSET(8) NUMBITS(1) [],
        /// FIFO 1 overrun interrupt enable
        FOVIE1 OFFSET(6) NUMBITS(1) [],
        /// FIFO 1 full interrupt enable
        FFIE1 OFFSET(5) NUMBITS(1) [],
        /// FIFO 1 message pending interrupt enable
        FMPIE1 OFFSET(4) NUMBITS(1) [],
        /// FIFO 0 overrun interrupt enable
        FOVIE0 OFFSET(3) NUMBITS(1) [],
        /// FIFO 0 full interrupt enable
        FFIE0 OFFSET(2) NUMBITS(1) [],
        /// FIFO 0 message pending interrupt enable
        FMPIE0 OFFSET(1) NUMBITS(1) [],
        /// Transmit mailbox empty interrupt enable
        TMEIE OFFSET(0) NUMBITS(1) []
    ],
    CAN_ESR [
        /// Receive error counter
        REC OFFSET(24) NUMBITS(8) [],
        /// Least significant byte of the 9-bit transmit error counter
        TEC OFFSET(16) NUMBITS(8) [],
        /// Last error code
        LEC OFFSET(4) NUMBITS(3) [
            NoError = 0,
            StuffError = 1,
            FormError = 2,
            AcknowledgmentError = 3,
            BitRecessiveError = 4,
            BitDominantError = 5,
            CrcError = 6,
            SetBySoftware = 7
        ],
        /// Bus-off flag
        BOFF OFFSET(2) NUMBITS(1) [],
        /// Error passive flag
        EPVF OFFSET(1) NUMBITS(1) [],
        /// Error warning flag
        EWGF OFFSET(0) NUMBITS(1) []
    ],
    CAN_BTR [
        /// Silent mode (debug)
        SILM OFFSET(31) NUMBITS(1) [],
        /// Loop back mode (debug)
        LBKM OFFSET(30) NUMBITS(1) [],
        /// Resynchronization jump width
        SJW OFFSET(24) NUMBITS(2) [],
        /// Time segment 2
        TS2 OFFSET(20) NUMBITS(3) [],
        /// Time segment 1
        TS1 OFFSET(16) NUMBITS(4) [],
        /// Baud rate prescaler
        BRP OFFSET(0) NUMBITS(10) []
    ],
    ///
    /// 
    /// CAN mailbox registers
    /// 
    /// 
    CAN_TIxR [
        /// Standard identifier or extended identifier
        STID OFFSET(21) NUMBITS(11) [],
        /// Extended identifier
        EXID OFFSET(3) NUMBITS(18) [],
        /// Identifier extension
        IDE OFFSET(2) NUMBITS(1) [],
        /// Remote transmission request
        RTR OFFSET(1) NUMBITS(1) [],
        /// Transmit mailbox request
        TXRQ OFFSET(0) NUMBITS(1) []
    ],
    CAN_TDTxR [
        /// Message time stamp
        TIME OFFSET(16) NUMBITS(16) [],
        /// Transmit global time
        TGT OFFSET(8) NUMBITS(1) [],
        /// Data length code
        DLC OFFSET(0) NUMBITS(4) []
    ],
    CAN_TDLxR [
        /// Data byte 3
        DATA3 OFFSET(24) NUMBITS(8) [],
        /// Data byte 2
        DATA2 OFFSET(16) NUMBITS(8) [],
        /// Data byte 1
        DATA1 OFFSET(8) NUMBITS(8) [],
        /// Data byte 0
        DATA0 OFFSET(0) NUMBITS(8) []
    ],
    CAN_TDHxR [
        /// Data byte 7
        DATA7 OFFSET(24) NUMBITS(8) [],
        /// Data byte 6
        DATA6 OFFSET(16) NUMBITS(8) [],
        /// Data byte 5
        DATA5 OFFSET(8) NUMBITS(8) [],
        /// Data byte 4
        DATA4 OFFSET(0) NUMBITS(8) []
    ],
    CAN_RIxR [
        /// Standard identifier or extended identifier
        STID OFFSET(21) NUMBITS(11) [],
        /// Extended identifier
        EXID OFFSET(3) NUMBITS(18) [],
        /// Identifier extension
        IDE OFFSET(2) NUMBITS(1) [],
        /// Remote transmission request
        RTR OFFSET(1) NUMBITS(1) []
    ],
    CAN_RDTxR [
        /// Message time stamp
        TIME OFFSET(16) NUMBITS(16) [],
        /// Filter match index
        FMI OFFSET(8) NUMBITS(8) [],
        /// Data length code
        DLC OFFSET(0) NUMBITS(4) []
    ],
    CAN_RDLxR [
        /// Data byte 3
        DATA3 OFFSET(24) NUMBITS(8) [],
        /// Data byte 2
        DATA2 OFFSET(16) NUMBITS(8) [],
        /// Data byte 1
        DATA1 OFFSET(8) NUMBITS(8) [],
        /// Data byte 0
        DATA0 OFFSET(0) NUMBITS(8) []
    ],
    CAN_RDHxR [
        /// Data byte 7
        DATA7 OFFSET(24) NUMBITS(8) [],
        /// Data byte 6
        DATA6 OFFSET(16) NUMBITS(8) [],
        /// Data byte 5
        DATA5 OFFSET(8) NUMBITS(8) [],
        /// Data byte 4
        DATA4 OFFSET(0) NUMBITS(8) []
    ],
    /// 
    /// 
    /// CAN filter registers
    /// 
    /// 
    CAN_FMR [
        /// CAN start bank
        CANSB OFFSET(8) NUMBITS(6) [],
        /// Filter initialization mode
        FINIT OFFSET(0) NUMBITS(1) []
    ],
    /// CAN filter mode register
    CAN_FM1R [
        /// Filter mode
        FBM OFFSET(0) NUMBITS(28) []
    ],
    CAN_FS1R [
        /// Filter scale configuration
        FSC OFFSET(0) NUMBITS(28) []
    ],
    CAN_FFA1R [
        /// Filter FIFO assignment for filter x
        FFA OFFSET(0) NUMBITS(28) []
    ],
    CAN_FA1R [
        /// Filter active
        FACT OFFSET(0) NUMBITS(28) []
    ],
    CAN_FiRx [
        /// Filter bits
        FB OFFSET(0) NUMBITS(32) []
    ]
];

/// 
const CAN1_BASE: StaticRef<CanRegisters> =
    unsafe { StaticRef::new(0x40006400 as *const CanRegisters) };
// const CAN2_BASE: StaticRef<CanRegisters> =
//     unsafe { StaticRef::new(0x40006800 as *const CanRegisters) };

#[derive(Copy, Clone, PartialEq)]
enum CanOperatingMode {
    Initialization,
    Normal,
    Sleep,
}

#[allow(dead_code)]
#[repr(u32)]
enum BitSegment1 {
    CanBtrTs1_1tq = 0b0000 << 16,
    CanBtrTs1_2tq = 0b0001 << 16,
    CanBtrTs1_3tq = 0b0010 << 16,
    CanBtrTs1_4tq = 0b0011 << 16,
    CanBtrTs1_5tq = 0b0100 << 16,
    CanBtrTs1_6tq = 0b0101 << 16,
    CanBtrTs1_7tq = 0b0110 << 16,
    CanBtrTs1_8tq = 0b0111 << 16,
    CanBtrTs1_9tq = 0b1000 << 16,
    CanBtrTs1_10tq = 0b1001 << 16,
    CanBtrTs1_11tq = 0b1010 << 16,
    CanBtrTs1_12tq = 0b1011 << 16,
    CanBtrTs1_13tq = 0b1100 << 16,
    CanBtrTs1_14tq = 0b1101 << 16,
    CanBtrTs1_15tq = 0b1110 << 16,
    CanBtrTs1_16tq = 0b1111 << 16,
}

#[allow(dead_code)]
#[repr(u32)]
enum BitSegment2 {
    CanBtrTs2_1tq = 0b0000 << 20,
    CanBtrTs2_2tq = 0b0001 << 20,
    CanBtrTs2_3tq = 0b0010 << 20,
    CanBtrTs2_4tq = 0b0011 << 20,
    CanBtrTs2_5tq = 0b0100 << 20,
    CanBtrTs2_6tq = 0b0101 << 20,
    CanBtrTs2_7tq = 0b0110 << 20,
    CanBtrTs2_8tq = 0b0111 << 20,
}

#[allow(dead_code)]
#[repr(u32)]
enum SynchronizationJumpWidth {
    CanBtrSjw1tq = 0b00 << 24,   
    CanBtrSjw2tq = 0b01 << 24,
    CanBtrSjw3tq = 0b10 << 24,
    CanBtrSjw4tq = 0b11 << 24,      
}

#[derive(Copy, Clone, PartialEq)]
pub enum CanInterruptMode {
    TransmitInterrupt,
    Fifo0Interrupt,
    Fifo1Interrupt,
    ErrorAndStatusChangeInterrupt,
}

pub struct Can<'a> {
    registers: StaticRef<CanRegisters>,
    clock: CanClock<'a>,
    operating_mode: Cell<CanOperatingMode>,
}

impl<'a> Can<'a> {
    pub const fn new(rcc: &'a rcc::Rcc) -> Can {
        Can {
            registers: CAN1_BASE,
            clock: CanClock(rcc::PeripheralClock::new(
                rcc::PeripheralClockType::APB1(rcc::PCLK1::CAN1),
                rcc,
            )),
            operating_mode: Cell::new(CanOperatingMode::Sleep),
        }
    }

    pub fn enable(&self) {
        // suppose we wake up after being in the sleep mode
        // sleep mode is by default set in the new function
        self.registers.can_mcr.modify(CAN_MCR::SLEEP::CLEAR);
        // wait sleep mode leave ack
        // while self.registers.can_msr.is_set(CAN_MSR::SLAK) {}

        self.registers.can_mcr.modify(CAN_MCR::INRQ::SET);

        // we wait for hardware to set the INAK bit
        while !self.registers.can_msr.is_set(CAN_MSR::INAK) {}
        self.operating_mode.set(CanOperatingMode::Initialization);

        self.registers.can_mcr.modify(CAN_MCR::ABOM::SET);
        self.registers.can_mcr.modify(CAN_MCR::NART::SET);

        // enter loopback mode - for debug
        self.registers.can_btr.modify(CAN_BTR::LBKM::SET);

        // set bit timing mode
        self.registers.can_btr.modify(CAN_BTR::TS1.val(BitSegment1::CanBtrTs1_6tq as u32));
        self.registers.can_btr.modify(CAN_BTR::TS2.val(BitSegment2::CanBtrTs2_2tq as u32));
        self.registers.can_btr.modify(CAN_BTR::SJW.val(SynchronizationJumpWidth::CanBtrSjw1tq as u32));
        self.registers.can_btr.modify(CAN_BTR::BRP.val(4));
    }

    pub fn config_filter(&self, filter_bank: u32, scale_32bits: bool, id_mask: bool, filter_fifo0: bool, filter_enable: bool) {
        let filter_number = 0x00000001 << filter_bank;
        hprintln!("filter_number este {}", filter_number).unwrap();
        self.registers.can_fmr.modify(CAN_FMR::FINIT::SET);

        self.registers.can_fa1r.modify(
            CAN_FA1R::FACT.val(
                self.registers.can_fa1r.read(CAN_FA1R::FACT) & !filter_number
            ));
        
        hprintln!("Suntem dupa set fa1r pe bitul 0 -> trebuie sa fie 0, adresa 0x4000661c").unwrap();
        if scale_32bits {
            self.registers.can_fs1r.modify(
                CAN_FS1R::FSC.val(
                    self.registers.can_fs1r.read(CAN_FS1R::FSC) | filter_number
                ));
        } else {
            self.registers.can_fs1r.modify(
                CAN_FS1R::FSC.val(
                    self.registers.can_fs1r.read(CAN_FS1R::FSC) & !filter_number
                ));
        }

        hprintln!("suntem dupa set fs1r pe bitul 0 -> trebuie sa fie 1, adresa 0x4000660C").unwrap();

        self.registers.can_f0r1.modify(CAN_FiRx::FB.val(0));

        hprintln!("dupa f0r1 -> trebuie sa fie 0, adresa 0x40006640").unwrap();
        self.registers.can_f0r2.modify(CAN_FiRx::FB.val(0));
        hprintln!("dupa f0r2 -> trebuie sa fie 0, adresa 0x40006644").unwrap();
        if id_mask {
            self.registers.can_fm1r.modify(
                CAN_FM1R::FBM.val(
                    self.registers.can_fm1r.read(CAN_FM1R::FBM) & !filter_number
                ));
        } else {
            self.registers.can_fm1r.modify(
                CAN_FM1R::FBM.val(
                    self.registers.can_fm1r.read(CAN_FM1R::FBM) | filter_number
                ));
        }

        hprintln!("dupa fm1r -> ar trebui sa fie 0, adresa 0x40006604").unwrap();

        if filter_fifo0 {
            self.registers.can_ffa1r.modify(
                CAN_FFA1R::FFA.val(
                    self.registers.can_ffa1r.read(CAN_FFA1R::FFA) & !filter_number
                ));
        } else {
            self.registers.can_ffa1r.modify(
                CAN_FFA1R::FFA.val(
                    self.registers.can_ffa1r.read(CAN_FFA1R::FFA) | filter_number
                ));
        }

        hprintln!("dupa ffa1r -> ar trebui sa fie 0, adresa 0x40006614").unwrap();
        if filter_enable {
            self.registers.can_fa1r.modify(
                CAN_FA1R::FACT.val(
                    self.registers.can_fa1r.read(CAN_FA1R::FACT) | filter_number
                ));
        }

        hprintln!("dupa fa1r -> ar trebui sa fie 1, adresa 0x4000661c").unwrap();
        self.registers.can_fmr.modify(CAN_FMR::FINIT::CLEAR);

        hprintln!("dupa finit, ar trebui sa fie 0x2A1C0E00, adresa 0x40006600").unwrap();
    }

    
    pub fn enter_normal_mode(&self) {
        
        // request to enter normal mode by clearing INRQ bit
        self.registers.can_mcr.modify(CAN_MCR::INRQ::CLEAR);
        // wait for INAK bit to be cleared
        // panic! ("here we are");
        while self.registers.can_msr.is_set(CAN_MSR::INAK) {}
        self.operating_mode.set(CanOperatingMode::Normal);

        // debug! ("we got here");
    }

    pub fn is_enabled_clock(&self) -> bool {
        self.clock.is_enabled()
    }

    pub fn enable_clock(&self) {
        // debug! ("enable clock\n");
        self.clock.enable();
    }

    pub fn disable_clock(&self) {
        self.clock.disable();
    }

    pub fn handle_transmit_interrupt(&self) {
        panic! ("transmit_interrupt_handler\n");
    }

    pub fn handle_fifo0_interrupt(&self) {
        panic! ("fifo0_interrupt_handler\n");
    }

    pub fn handle_fifo1_interrupt(&self) {
        panic! ("fifo1_interrupt_handler\n");
    }

    pub fn handle_error_status_interrupt(&self) {
        panic! ("error_and_status_change interrupt\n");
    }

    pub fn enable_irq(&self, interrupt: CanInterruptMode) {
        match interrupt {
            CanInterruptMode::TransmitInterrupt => {
                debug! ("enable irq transmit");
                self.registers.can_ier.modify(CAN_IER::TMEIE::SET);
            },
            CanInterruptMode::Fifo0Interrupt => {
                debug! ("enable irq fifo0");
                self.registers.can_ier.modify(CAN_IER::FMPIE0::SET);
                self.registers.can_ier.modify(CAN_IER::FFIE0::SET);
                self.registers.can_ier.modify(CAN_IER::FOVIE0::SET);
            },
            CanInterruptMode::Fifo1Interrupt => {
                debug! ("enable irq fifo1");
                self.registers.can_ier.modify(CAN_IER::FMPIE1::SET);
                self.registers.can_ier.modify(CAN_IER::FFIE1::SET);
                self.registers.can_ier.modify(CAN_IER::FOVIE1::SET);
            },
            CanInterruptMode::ErrorAndStatusChangeInterrupt => {
                debug! ("enable irq error and status change");
                self.registers.can_ier.modify(CAN_IER::ERRIE::SET);
                self.registers.can_ier.modify(CAN_IER::EWGIE::SET);
                self.registers.can_ier.modify(CAN_IER::EPVIE::SET);
                self.registers.can_ier.modify(CAN_IER::BOFIE::SET);
                self.registers.can_ier.modify(CAN_IER::LECIE::SET);
                self.registers.can_ier.modify(CAN_IER::WKUIE::SET);
                self.registers.can_ier.modify(CAN_IER::SLKIE::SET);
            },
        }
    }

    pub fn disable_irq(&self, interrupt: CanInterruptMode) {
        match interrupt {
            CanInterruptMode::TransmitInterrupt => {
                self.registers.can_ier.modify(CAN_IER::TMEIE::CLEAR);
            },
            CanInterruptMode::Fifo0Interrupt => {
                self.registers.can_ier.modify(CAN_IER::FMPIE0::CLEAR);
                self.registers.can_ier.modify(CAN_IER::FFIE0::CLEAR);
                self.registers.can_ier.modify(CAN_IER::FOVIE0::CLEAR);
            },
            CanInterruptMode::Fifo1Interrupt => {
                self.registers.can_ier.modify(CAN_IER::FMPIE1::CLEAR);
                self.registers.can_ier.modify(CAN_IER::FFIE1::CLEAR);
                self.registers.can_ier.modify(CAN_IER::FOVIE1::CLEAR);
            },
            CanInterruptMode::ErrorAndStatusChangeInterrupt => {
                self.registers.can_ier.modify(CAN_IER::ERRIE::CLEAR);
                self.registers.can_ier.modify(CAN_IER::EWGIE::CLEAR);
                self.registers.can_ier.modify(CAN_IER::EPVIE::CLEAR);
                self.registers.can_ier.modify(CAN_IER::BOFIE::CLEAR);
                self.registers.can_ier.modify(CAN_IER::LECIE::CLEAR);
                self.registers.can_ier.modify(CAN_IER::WKUIE::CLEAR);
                self.registers.can_ier.modify(CAN_IER::SLKIE::CLEAR);
            },
        }
    }

    pub fn enable_irqs(&self) {
        self.enable_irq(CanInterruptMode::TransmitInterrupt);
        self.enable_irq(CanInterruptMode::Fifo0Interrupt);
        self.enable_irq(CanInterruptMode::Fifo1Interrupt);
        self.enable_irq(CanInterruptMode::ErrorAndStatusChangeInterrupt);
    }

    pub fn disable_irqs(&self) {
        self.disable_irq(CanInterruptMode::TransmitInterrupt);
        self.disable_irq(CanInterruptMode::Fifo0Interrupt);
        self.disable_irq(CanInterruptMode::Fifo1Interrupt);
        self.disable_irq(CanInterruptMode::ErrorAndStatusChangeInterrupt);
    }

    pub fn reset(&self) {
        self.registers.can_mcr.modify(CAN_MCR::RESET::SET);
        self.operating_mode.set(CanOperatingMode::Sleep);
    }

    
}

struct CanClock<'a>(rcc::PeripheralClock<'a>);

impl ClockInterface for CanClock<'_> {
    fn is_enabled(&self) -> bool {
        self.0.is_enabled()
    }

    fn enable(&self) {
        self.0.enable();
    }

    fn disable(&self) {
        self.0.disable();
    }
}