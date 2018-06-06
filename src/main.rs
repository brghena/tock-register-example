#![feature(const_fn)]

#[macro_use]
extern crate tock_regs;

use tock_regs::regs::{ReadOnly, ReadWrite, WriteOnly};

#[repr(C)]
struct Registers {
    // Control register: read-write
    // The 'Control' parameter constrains this register to only use fields from
    // a certain group (defined below in the bitfields section).
    cr: ReadWrite<u8, Control::Register>,

    // Status register: read-only
    s: ReadOnly<u8, Status::Register>,

    // Registers can be bytes, halfwords, or words:
    // Note that the second type parameter can be omitted, meaning that there
    // are no bitfields defined for these registers.
    byte0: WriteOnly<u8, InterruptFlags::Register>
}

register_bitfields! [
    // First parameter is the register width for the bitfields. Can be u8, u16,
    // or u32.
    u8,

    // Each subsequent parameter is a register abbreviation, its descriptive
    // name, and its associated bitfields.
    // The descriptive name defines this 'group' of bitfields. Only registers
    // defined as ReadWrite<_, Control::Register> can use these bitfields.
    Control [
        // Bitfields are defined as:
        // name OFFSET(shift) NUMBITS(num) [ /* optional values */ ]

        // This is a two-bit field which includes bits 4 and 5
        RANGE OFFSET(4) NUMBITS(3) [
            // Each of these defines a name for a value that the bitfield can be
            // written with or matched against. Note that this set is not exclusive--
            // the field can still be written with arbitrary constants.
            VeryHigh = 0,
            High = 1,
            Low = 2
        ],

        // A common case is single-bit bitfields, which usually just mean
        // 'enable' or 'disable' something.
        EN  OFFSET(3) NUMBITS(1) [],
        INT OFFSET(2) NUMBITS(1) []
    ],

    // Another example:
    // Status register
    Status [
        TXCOMPLETE  OFFSET(0) NUMBITS(1) [],
        TXINTERRUPT OFFSET(1) NUMBITS(1) [],
        RXCOMPLETE  OFFSET(2) NUMBITS(1) [],
        RXINTERRUPT OFFSET(3) NUMBITS(1) [],
        MODE        OFFSET(4) NUMBITS(3) [
            FullDuplex = 0,
            HalfDuplex = 1,
            Loopback = 2,
            Disabled = 3
        ],
        ERRORCOUNT OFFSET(7) NUMBITS(1) []
    ],

    // In a simple case, offset can just be a number, and the number of bits
    // is set to 1:
    InterruptFlags [
        OVRES    3,
        MODF     2,
        TDRE     1,
        RDRF     0
    ]
];

fn main() {
    let registers: *const Registers = 0xffff_ffff as *const Registers;
    unsafe { (*registers).cr.modify(Control::RANGE::VeryHigh); }
}
