use std::ops::Add;

use crate::reg;
use crate::memory;
use crate::reg::FlagsRegister;

use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CpuTest {
    pub name: String,
    #[serde(rename = "initial")]
    pub initial_state: CpuState,
    #[serde(rename = "final")]
    pub final_state: CpuState,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CpuState {
    pc: u16,
    sp: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    ime: u8,
    #[serde(default)]
    ie: u8,
    #[serde(default)]
    ei: u8,
    ram: Vec<[u16; 2]>,
}

pub enum Target {
    Reg8(Reg8),
    Reg16(Reg16),
    Reg16Indirect(Reg16),
    Value,
}

pub enum Reg8 {
    A, B, C, D, E, H, L, D8, HLI, BCI, DEI, HLII, HLDI, D16I, CI, D8I
}

// af, bc, de, hl
pub enum Reg16 {
    AF, BC, DE, HL, SP, D16, I16
}

pub enum StackTarget {
    AF, BC, DE, HL
}

pub enum AddHLTarget {
    BC, DE, HL, SP
}

pub enum Instruction {
    Add(Target),
    AddHL(AddHLTarget),
    Adc(Target),
    Sub(Target),
    Sbc(Target),
    Or(Target),
    And(Target),
    Xor(Target),
    Cp(Target),
    Inc(Target),
    Dec(Target),
    Rlc(Target),
    Rrc(Target),
    Rl(Target),
    Rr(Target),
    Sla(Target),
    Sra(Target),
    Swap(Target),
    Srl(Target),
    Bit0(Target),
    Bit1(Target),
    Bit2(Target),
    Bit3(Target),
    Bit4(Target),
    Bit5(Target),
    Bit6(Target),
    Bit7(Target),
    Res0(Target),
    Res1(Target),
    Res2(Target),
    Res3(Target),
    Res4(Target),
    Res5(Target),
    Res6(Target),
    Res7(Target),
    Set0(Target),
    Set1(Target),
    Set2(Target),
    Set3(Target),
    Set4(Target),
    Set5(Target),
    Set6(Target),
    Set7(Target),
    Cpl,
    Ccf,
    Scf,
    Nop,
    Rlca,
    Rla,
    Rrca,
    Rra,

    HALT,
    DI,

    JP(JumpTest),
    JR(JumpTest),
    JPHL,

    LD(LoadType),
    POP(StackTarget),
    PUSH(StackTarget),
    CALL(JumpTest),
    RET(JumpTest),
}

pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

pub enum LoadType {
    Word(Reg16, Reg16),
    Byte(Reg8, Reg8),
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::Rlc(Target::Reg8(Reg8::B))),
            0x01 => Some(Instruction::Rlc(Target::Reg8(Reg8::C))),
            0x02 => Some(Instruction::Rlc(Target::Reg8(Reg8::D))),
            0x03 => Some(Instruction::Rlc(Target::Reg8(Reg8::E))),
            0x04 => Some(Instruction::Rlc(Target::Reg8(Reg8::H))),
            0x05 => Some(Instruction::Rlc(Target::Reg8(Reg8::L))),
            0x06 => Some(Instruction::Rlc(Target::Reg16Indirect(Reg16::HL))),
            0x07 => Some(Instruction::Rlc(Target::Reg8(Reg8::A))),

            0x08 => Some(Instruction::Rrc(Target::Reg8(Reg8::B))),
            0x09 => Some(Instruction::Rrc(Target::Reg8(Reg8::C))),
            0x0A => Some(Instruction::Rrc(Target::Reg8(Reg8::D))),
            0x0B => Some(Instruction::Rrc(Target::Reg8(Reg8::E))),
            0x0C => Some(Instruction::Rrc(Target::Reg8(Reg8::H))),
            0x0D => Some(Instruction::Rrc(Target::Reg8(Reg8::L))),
            0x0E => Some(Instruction::Rrc(Target::Reg16Indirect(Reg16::HL))),
            0x0F => Some(Instruction::Rrc(Target::Reg8(Reg8::A))),

            0x10 => Some(Instruction::Rl(Target::Reg8(Reg8::B))),
            0x11 => Some(Instruction::Rl(Target::Reg8(Reg8::C))),
            0x12 => Some(Instruction::Rl(Target::Reg8(Reg8::D))),
            0x13 => Some(Instruction::Rl(Target::Reg8(Reg8::E))),
            0x14 => Some(Instruction::Rl(Target::Reg8(Reg8::H))),
            0x15 => Some(Instruction::Rl(Target::Reg8(Reg8::L))),
            0x16 => Some(Instruction::Rl(Target::Reg16Indirect(Reg16::HL))),
            0x17 => Some(Instruction::Rl(Target::Reg8(Reg8::A))),

            0x18 => Some(Instruction::Rr(Target::Reg8(Reg8::B))),
            0x19 => Some(Instruction::Rr(Target::Reg8(Reg8::C))),
            0x1A => Some(Instruction::Rr(Target::Reg8(Reg8::D))),
            0x1B => Some(Instruction::Rr(Target::Reg8(Reg8::E))),
            0x1C => Some(Instruction::Rr(Target::Reg8(Reg8::H))),
            0x1D => Some(Instruction::Rr(Target::Reg8(Reg8::L))),
            0x1E => Some(Instruction::Rr(Target::Reg16Indirect(Reg16::HL))),
            0x1F => Some(Instruction::Rr(Target::Reg8(Reg8::A))),

            0x20 => Some(Instruction::Sla(Target::Reg8(Reg8::B))),
            0x21 => Some(Instruction::Sla(Target::Reg8(Reg8::C))),
            0x22 => Some(Instruction::Sla(Target::Reg8(Reg8::D))),
            0x23 => Some(Instruction::Sla(Target::Reg8(Reg8::E))),
            0x24 => Some(Instruction::Sla(Target::Reg8(Reg8::H))),
            0x25 => Some(Instruction::Sla(Target::Reg8(Reg8::L))),
            0x26 => Some(Instruction::Sla(Target::Reg16Indirect(Reg16::HL))),
            0x27 => Some(Instruction::Sla(Target::Reg8(Reg8::A))),

            0x28 => Some(Instruction::Sra(Target::Reg8(Reg8::B))),
            0x29 => Some(Instruction::Sra(Target::Reg8(Reg8::C))),
            0x2A => Some(Instruction::Sra(Target::Reg8(Reg8::D))),
            0x2B => Some(Instruction::Sra(Target::Reg8(Reg8::E))),
            0x2C => Some(Instruction::Sra(Target::Reg8(Reg8::H))),
            0x2D => Some(Instruction::Sra(Target::Reg8(Reg8::L))),
            0x2E => Some(Instruction::Sra(Target::Reg16Indirect(Reg16::HL))),
            0x2F => Some(Instruction::Sra(Target::Reg8(Reg8::A))),

            0x30 => Some(Instruction::Swap(Target::Reg8(Reg8::B))),
            0x31 => Some(Instruction::Swap(Target::Reg8(Reg8::C))),
            0x32 => Some(Instruction::Swap(Target::Reg8(Reg8::D))),
            0x33 => Some(Instruction::Swap(Target::Reg8(Reg8::E))),
            0x34 => Some(Instruction::Swap(Target::Reg8(Reg8::H))),
            0x35 => Some(Instruction::Swap(Target::Reg8(Reg8::L))),
            0x36 => Some(Instruction::Swap(Target::Reg16Indirect(Reg16::HL))),
            0x37 => Some(Instruction::Swap(Target::Reg8(Reg8::A))),

            0x38 => Some(Instruction::Srl(Target::Reg8(Reg8::B))),
            0x39 => Some(Instruction::Srl(Target::Reg8(Reg8::C))),
            0x3A => Some(Instruction::Srl(Target::Reg8(Reg8::D))),
            0x3B => Some(Instruction::Srl(Target::Reg8(Reg8::E))),
            0x3C => Some(Instruction::Srl(Target::Reg8(Reg8::H))),
            0x3D => Some(Instruction::Srl(Target::Reg8(Reg8::L))),
            0x3E => Some(Instruction::Srl(Target::Reg16Indirect(Reg16::HL))),
            0x3F => Some(Instruction::Srl(Target::Reg8(Reg8::A))),

            /* BIT */
            0x40 => Some(Instruction::Bit0(Target::Reg8(Reg8::B))),
            0x41 => Some(Instruction::Bit0(Target::Reg8(Reg8::C))),
            0x42 => Some(Instruction::Bit0(Target::Reg8(Reg8::D))),
            0x43 => Some(Instruction::Bit0(Target::Reg8(Reg8::E))),
            0x44 => Some(Instruction::Bit0(Target::Reg8(Reg8::H))),
            0x45 => Some(Instruction::Bit0(Target::Reg8(Reg8::L))),
            0x46 => Some(Instruction::Bit0(Target::Reg16Indirect(Reg16::HL))),
            0x47 => Some(Instruction::Bit0(Target::Reg8(Reg8::A))),

            0x48 => Some(Instruction::Bit1(Target::Reg8(Reg8::B))),
            0x49 => Some(Instruction::Bit1(Target::Reg8(Reg8::C))),
            0x4A => Some(Instruction::Bit1(Target::Reg8(Reg8::D))),
            0x4B => Some(Instruction::Bit1(Target::Reg8(Reg8::E))),
            0x4C => Some(Instruction::Bit1(Target::Reg8(Reg8::H))),
            0x4D => Some(Instruction::Bit1(Target::Reg8(Reg8::L))),
            0x4E => Some(Instruction::Bit1(Target::Reg16Indirect(Reg16::HL))),
            0x4F => Some(Instruction::Bit1(Target::Reg8(Reg8::A))),

            0x50 => Some(Instruction::Bit2(Target::Reg8(Reg8::B))),
            0x51 => Some(Instruction::Bit2(Target::Reg8(Reg8::C))),
            0x52 => Some(Instruction::Bit2(Target::Reg8(Reg8::D))),
            0x53 => Some(Instruction::Bit2(Target::Reg8(Reg8::E))),
            0x54 => Some(Instruction::Bit2(Target::Reg8(Reg8::H))),
            0x55 => Some(Instruction::Bit2(Target::Reg8(Reg8::L))),
            0x56 => Some(Instruction::Bit2(Target::Reg16Indirect(Reg16::HL))),
            0x57 => Some(Instruction::Bit2(Target::Reg8(Reg8::A))),

            0x58 => Some(Instruction::Bit3(Target::Reg8(Reg8::B))),
            0x59 => Some(Instruction::Bit3(Target::Reg8(Reg8::C))),
            0x5A => Some(Instruction::Bit3(Target::Reg8(Reg8::D))),
            0x5B => Some(Instruction::Bit3(Target::Reg8(Reg8::E))),
            0x5C => Some(Instruction::Bit3(Target::Reg8(Reg8::H))),
            0x5D => Some(Instruction::Bit3(Target::Reg8(Reg8::L))),
            0x5E => Some(Instruction::Bit3(Target::Reg16Indirect(Reg16::HL))),
            0x5F => Some(Instruction::Bit3(Target::Reg8(Reg8::A))),

            0x60 => Some(Instruction::Bit4(Target::Reg8(Reg8::B))),
            0x61 => Some(Instruction::Bit4(Target::Reg8(Reg8::C))),
            0x62 => Some(Instruction::Bit4(Target::Reg8(Reg8::D))),
            0x63 => Some(Instruction::Bit4(Target::Reg8(Reg8::E))),
            0x64 => Some(Instruction::Bit4(Target::Reg8(Reg8::H))),
            0x65 => Some(Instruction::Bit4(Target::Reg8(Reg8::L))),
            0x66 => Some(Instruction::Bit4(Target::Reg16Indirect(Reg16::HL))),
            0x67 => Some(Instruction::Bit4(Target::Reg8(Reg8::A))),

            0x68 => Some(Instruction::Bit5(Target::Reg8(Reg8::B))),
            0x69 => Some(Instruction::Bit5(Target::Reg8(Reg8::C))),
            0x6A => Some(Instruction::Bit5(Target::Reg8(Reg8::D))),
            0x6B => Some(Instruction::Bit5(Target::Reg8(Reg8::E))),
            0x6C => Some(Instruction::Bit5(Target::Reg8(Reg8::H))),
            0x6D => Some(Instruction::Bit5(Target::Reg8(Reg8::L))),
            0x6E => Some(Instruction::Bit5(Target::Reg16Indirect(Reg16::HL))),
            0x6F => Some(Instruction::Bit5(Target::Reg8(Reg8::A))),

            0x70 => Some(Instruction::Bit6(Target::Reg8(Reg8::B))),
            0x71 => Some(Instruction::Bit6(Target::Reg8(Reg8::C))),
            0x72 => Some(Instruction::Bit6(Target::Reg8(Reg8::D))),
            0x73 => Some(Instruction::Bit6(Target::Reg8(Reg8::E))),
            0x74 => Some(Instruction::Bit6(Target::Reg8(Reg8::H))),
            0x75 => Some(Instruction::Bit6(Target::Reg8(Reg8::L))),
            0x76 => Some(Instruction::Bit6(Target::Reg16Indirect(Reg16::HL))),
            0x77 => Some(Instruction::Bit6(Target::Reg8(Reg8::A))),

            0x78 => Some(Instruction::Bit7(Target::Reg8(Reg8::B))),
            0x79 => Some(Instruction::Bit7(Target::Reg8(Reg8::C))),
            0x7A => Some(Instruction::Bit7(Target::Reg8(Reg8::D))),
            0x7B => Some(Instruction::Bit7(Target::Reg8(Reg8::E))),
            0x7C => Some(Instruction::Bit7(Target::Reg8(Reg8::H))),
            0x7D => Some(Instruction::Bit7(Target::Reg8(Reg8::L))),
            0x7E => Some(Instruction::Bit7(Target::Reg16Indirect(Reg16::HL))),
            0x7F => Some(Instruction::Bit7(Target::Reg8(Reg8::A))),

            /* RES */
            0x80 => Some(Instruction::Res0(Target::Reg8(Reg8::B))),
            0x81 => Some(Instruction::Res0(Target::Reg8(Reg8::C))),
            0x82 => Some(Instruction::Res0(Target::Reg8(Reg8::D))),
            0x83 => Some(Instruction::Res0(Target::Reg8(Reg8::E))),
            0x84 => Some(Instruction::Res0(Target::Reg8(Reg8::H))),
            0x85 => Some(Instruction::Res0(Target::Reg8(Reg8::L))),
            0x86 => Some(Instruction::Res0(Target::Reg16Indirect(Reg16::HL))),
            0x87 => Some(Instruction::Res0(Target::Reg8(Reg8::A))),

            0x88 => Some(Instruction::Res1(Target::Reg8(Reg8::B))),
            0x89 => Some(Instruction::Res1(Target::Reg8(Reg8::C))),
            0x8A => Some(Instruction::Res1(Target::Reg8(Reg8::D))),
            0x8B => Some(Instruction::Res1(Target::Reg8(Reg8::E))),
            0x8C => Some(Instruction::Res1(Target::Reg8(Reg8::H))),
            0x8D => Some(Instruction::Res1(Target::Reg8(Reg8::L))),
            0x8E => Some(Instruction::Res1(Target::Reg16Indirect(Reg16::HL))),
            0x8F => Some(Instruction::Res1(Target::Reg8(Reg8::A))),

            0x90 => Some(Instruction::Res2(Target::Reg8(Reg8::B))),
            0x91 => Some(Instruction::Res2(Target::Reg8(Reg8::C))),
            0x92 => Some(Instruction::Res2(Target::Reg8(Reg8::D))),
            0x93 => Some(Instruction::Res2(Target::Reg8(Reg8::E))),
            0x94 => Some(Instruction::Res2(Target::Reg8(Reg8::H))),
            0x95 => Some(Instruction::Res2(Target::Reg8(Reg8::L))),
            0x96 => Some(Instruction::Res2(Target::Reg16Indirect(Reg16::HL))),
            0x97 => Some(Instruction::Res2(Target::Reg8(Reg8::A))),

            0x98 => Some(Instruction::Res3(Target::Reg8(Reg8::B))),
            0x99 => Some(Instruction::Res3(Target::Reg8(Reg8::C))),
            0x9A => Some(Instruction::Res3(Target::Reg8(Reg8::D))),
            0x9B => Some(Instruction::Res3(Target::Reg8(Reg8::E))),
            0x9C => Some(Instruction::Res3(Target::Reg8(Reg8::H))),
            0x9D => Some(Instruction::Res3(Target::Reg8(Reg8::L))),
            0x9E => Some(Instruction::Res3(Target::Reg16Indirect(Reg16::HL))),
            0x9F => Some(Instruction::Res3(Target::Reg8(Reg8::A))),

            0xA0 => Some(Instruction::Res4(Target::Reg8(Reg8::B))),
            0xA1 => Some(Instruction::Res4(Target::Reg8(Reg8::C))),
            0xA2 => Some(Instruction::Res4(Target::Reg8(Reg8::D))),
            0xA3 => Some(Instruction::Res4(Target::Reg8(Reg8::E))),
            0xA4 => Some(Instruction::Res4(Target::Reg8(Reg8::H))),
            0xA5 => Some(Instruction::Res4(Target::Reg8(Reg8::L))),
            0xA6 => Some(Instruction::Res4(Target::Reg16Indirect(Reg16::HL))),
            0xA7 => Some(Instruction::Res4(Target::Reg8(Reg8::A))),

            0xA8 => Some(Instruction::Res5(Target::Reg8(Reg8::B))),
            0xA9 => Some(Instruction::Res5(Target::Reg8(Reg8::C))),
            0xAA => Some(Instruction::Res5(Target::Reg8(Reg8::D))),
            0xAB => Some(Instruction::Res5(Target::Reg8(Reg8::E))),
            0xAC => Some(Instruction::Res5(Target::Reg8(Reg8::H))),
            0xAD => Some(Instruction::Res5(Target::Reg8(Reg8::L))),
            0xAE => Some(Instruction::Res5(Target::Reg16Indirect(Reg16::HL))),
            0xAF => Some(Instruction::Res5(Target::Reg8(Reg8::A))),

            0xB0 => Some(Instruction::Res6(Target::Reg8(Reg8::B))),
            0xB1 => Some(Instruction::Res6(Target::Reg8(Reg8::C))),
            0xB2 => Some(Instruction::Res6(Target::Reg8(Reg8::D))),
            0xB3 => Some(Instruction::Res6(Target::Reg8(Reg8::E))),
            0xB4 => Some(Instruction::Res6(Target::Reg8(Reg8::H))),
            0xB5 => Some(Instruction::Res6(Target::Reg8(Reg8::L))),
            0xB6 => Some(Instruction::Res6(Target::Reg16Indirect(Reg16::HL))),
            0xB7 => Some(Instruction::Res6(Target::Reg8(Reg8::A))),

            0xB8 => Some(Instruction::Res7(Target::Reg8(Reg8::B))),
            0xB9 => Some(Instruction::Res7(Target::Reg8(Reg8::C))),
            0xBA => Some(Instruction::Res7(Target::Reg8(Reg8::D))),
            0xBB => Some(Instruction::Res7(Target::Reg8(Reg8::E))),
            0xBC => Some(Instruction::Res7(Target::Reg8(Reg8::H))),
            0xBD => Some(Instruction::Res7(Target::Reg8(Reg8::L))),
            0xBE => Some(Instruction::Res7(Target::Reg16Indirect(Reg16::HL))),
            0xBF => Some(Instruction::Res7(Target::Reg8(Reg8::A))),

            /* SET */
            0xC0 => Some(Instruction::Set0(Target::Reg8(Reg8::B))),
            0xC1 => Some(Instruction::Set0(Target::Reg8(Reg8::C))),
            0xC2 => Some(Instruction::Set0(Target::Reg8(Reg8::D))),
            0xC3 => Some(Instruction::Set0(Target::Reg8(Reg8::E))),
            0xC4 => Some(Instruction::Set0(Target::Reg8(Reg8::H))),
            0xC5 => Some(Instruction::Set0(Target::Reg8(Reg8::L))),
            0xC6 => Some(Instruction::Set0(Target::Reg16Indirect(Reg16::HL))),
            0xC7 => Some(Instruction::Set0(Target::Reg8(Reg8::A))),

            0xC8 => Some(Instruction::Set1(Target::Reg8(Reg8::B))),
            0xC9 => Some(Instruction::Set1(Target::Reg8(Reg8::C))),
            0xCA => Some(Instruction::Set1(Target::Reg8(Reg8::D))),
            0xCB => Some(Instruction::Set1(Target::Reg8(Reg8::E))),
            0xCC => Some(Instruction::Set1(Target::Reg8(Reg8::H))),
            0xCD => Some(Instruction::Set1(Target::Reg8(Reg8::L))),
            0xCE => Some(Instruction::Set1(Target::Reg16Indirect(Reg16::HL))),
            0xCF => Some(Instruction::Set1(Target::Reg8(Reg8::A))),

            0xD0 => Some(Instruction::Set2(Target::Reg8(Reg8::B))),
            0xD1 => Some(Instruction::Set2(Target::Reg8(Reg8::C))),
            0xD2 => Some(Instruction::Set2(Target::Reg8(Reg8::D))),
            0xD3 => Some(Instruction::Set2(Target::Reg8(Reg8::E))),
            0xD4 => Some(Instruction::Set2(Target::Reg8(Reg8::H))),
            0xD5 => Some(Instruction::Set2(Target::Reg8(Reg8::L))),
            0xD6 => Some(Instruction::Set2(Target::Reg16Indirect(Reg16::HL))),
            0xD7 => Some(Instruction::Set2(Target::Reg8(Reg8::A))),

            0xD8 => Some(Instruction::Set3(Target::Reg8(Reg8::B))),
            0xD9 => Some(Instruction::Set3(Target::Reg8(Reg8::C))),
            0xDA => Some(Instruction::Set3(Target::Reg8(Reg8::D))),
            0xDB => Some(Instruction::Set3(Target::Reg8(Reg8::E))),
            0xDC => Some(Instruction::Set3(Target::Reg8(Reg8::H))),
            0xDD => Some(Instruction::Set3(Target::Reg8(Reg8::L))),
            0xDE => Some(Instruction::Set3(Target::Reg16Indirect(Reg16::HL))),
            0xDF => Some(Instruction::Set3(Target::Reg8(Reg8::A))),

            0xE0 => Some(Instruction::Set4(Target::Reg8(Reg8::B))),
            0xE1 => Some(Instruction::Set4(Target::Reg8(Reg8::C))),
            0xE2 => Some(Instruction::Set4(Target::Reg8(Reg8::D))),
            0xE3 => Some(Instruction::Set4(Target::Reg8(Reg8::E))),
            0xE4 => Some(Instruction::Set4(Target::Reg8(Reg8::H))),
            0xE5 => Some(Instruction::Set4(Target::Reg8(Reg8::L))),
            0xE6 => Some(Instruction::Set4(Target::Reg16Indirect(Reg16::HL))),
            0xE7 => Some(Instruction::Set4(Target::Reg8(Reg8::A))),

            0xE8 => Some(Instruction::Set5(Target::Reg8(Reg8::B))),
            0xE9 => Some(Instruction::Set5(Target::Reg8(Reg8::C))),
            0xEA => Some(Instruction::Set5(Target::Reg8(Reg8::D))),
            0xEB => Some(Instruction::Set5(Target::Reg8(Reg8::E))),
            0xEC => Some(Instruction::Set5(Target::Reg8(Reg8::H))),
            0xED => Some(Instruction::Set5(Target::Reg8(Reg8::L))),
            0xEE => Some(Instruction::Set5(Target::Reg16Indirect(Reg16::HL))),
            0xEF => Some(Instruction::Set5(Target::Reg8(Reg8::A))),

            0xF0 => Some(Instruction::Set6(Target::Reg8(Reg8::B))),
            0xF1 => Some(Instruction::Set6(Target::Reg8(Reg8::C))),
            0xF2 => Some(Instruction::Set6(Target::Reg8(Reg8::D))),
            0xF3 => Some(Instruction::Set6(Target::Reg8(Reg8::E))),
            0xF4 => Some(Instruction::Set6(Target::Reg8(Reg8::H))),
            0xF5 => Some(Instruction::Set6(Target::Reg8(Reg8::L))),
            0xF6 => Some(Instruction::Set6(Target::Reg16Indirect(Reg16::HL))),
            0xF7 => Some(Instruction::Set6(Target::Reg8(Reg8::A))),

            0xF8 => Some(Instruction::Set7(Target::Reg8(Reg8::B))),
            0xF9 => Some(Instruction::Set7(Target::Reg8(Reg8::C))),
            0xFA => Some(Instruction::Set7(Target::Reg8(Reg8::D))),
            0xFB => Some(Instruction::Set7(Target::Reg8(Reg8::E))),
            0xFC => Some(Instruction::Set7(Target::Reg8(Reg8::H))),
            0xFD => Some(Instruction::Set7(Target::Reg8(Reg8::L))),
            0xFE => Some(Instruction::Set7(Target::Reg16Indirect(Reg16::HL))),
            0xFF => Some(Instruction::Set7(Target::Reg8(Reg8::A)))
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::Nop),
            0x07 => Some(Instruction::Rlca),
            0x0F => Some(Instruction::Rrca),
            0x17 => Some(Instruction::Rla),
            0x1F => Some(Instruction::Rra),
            /* DAA 0x27 => */
            0x2F => Some(Instruction::Cpl),
            0x37 => Some(Instruction::Scf),
            0x3F => Some(Instruction::Ccf),

            0x04 => Some(Instruction::Inc(Target::Reg8(Reg8::B))),
            0x0C => Some(Instruction::Inc(Target::Reg8(Reg8::C))),
            0x14 => Some(Instruction::Inc(Target::Reg8(Reg8::D))),
            0x1C => Some(Instruction::Inc(Target::Reg8(Reg8::E))),
            0x24 => Some(Instruction::Inc(Target::Reg8(Reg8::H))),
            0x2C => Some(Instruction::Inc(Target::Reg8(Reg8::L))),
            0x34 => Some(Instruction::Inc(Target::Reg16Indirect(Reg16::HL))),
            0x3C => Some(Instruction::Inc(Target::Reg8(Reg8::A))),

            0x03 => Some(Instruction::Inc(Target::Reg16(Reg16::BC))),
            0x13 => Some(Instruction::Inc(Target::Reg16(Reg16::DE))),
            0x23 => Some(Instruction::Inc(Target::Reg16(Reg16::HL))),
            0x33 => Some(Instruction::Inc(Target::Reg16(Reg16::SP))),

            0x05 => Some(Instruction::Dec(Target::Reg8(Reg8::B))),
            0x0D => Some(Instruction::Dec(Target::Reg8(Reg8::C))),
            0x15 => Some(Instruction::Dec(Target::Reg8(Reg8::D))),
            0x1D => Some(Instruction::Dec(Target::Reg8(Reg8::E))),
            0x25 => Some(Instruction::Dec(Target::Reg8(Reg8::H))),
            0x2D => Some(Instruction::Dec(Target::Reg8(Reg8::L))),
            0x35 => Some(Instruction::Dec(Target::Reg16Indirect(Reg16::HL))),
            0x3D => Some(Instruction::Dec(Target::Reg8(Reg8::A))),

            0x80 => Some(Instruction::Add(Target::Reg8(Reg8::B))),
            0x81 => Some(Instruction::Add(Target::Reg8(Reg8::C))),
            0x82 => Some(Instruction::Add(Target::Reg8(Reg8::D))),
            0x83 => Some(Instruction::Add(Target::Reg8(Reg8::E))),
            0x84 => Some(Instruction::Add(Target::Reg8(Reg8::H))),
            0x85 => Some(Instruction::Add(Target::Reg8(Reg8::L))),
            0x86 => Some(Instruction::Add(Target::Reg16Indirect(Reg16::HL))),
            0x87 => Some(Instruction::Add(Target::Reg8(Reg8::A))),
            0xC6 => Some(Instruction::Add(Target::Value)),
            0x09 => Some(Instruction::AddHL(AddHLTarget::BC)),
            0x19 => Some(Instruction::AddHL(AddHLTarget::DE)),
            0x29 => Some(Instruction::AddHL(AddHLTarget::HL)),
            0x39 => Some(Instruction::AddHL(AddHLTarget::SP)),

            0x88 => Some(Instruction::Adc(Target::Reg8(Reg8::B))),
            0x89 => Some(Instruction::Adc(Target::Reg8(Reg8::C))),
            0x8A => Some(Instruction::Adc(Target::Reg8(Reg8::D))),
            0x8B => Some(Instruction::Adc(Target::Reg8(Reg8::E))),
            0x8C => Some(Instruction::Adc(Target::Reg8(Reg8::H))),
            0x8D => Some(Instruction::Adc(Target::Reg8(Reg8::L))),
            0x8E => Some(Instruction::Adc(Target::Reg16Indirect(Reg16::HL))),
            0x8F => Some(Instruction::Adc(Target::Reg8(Reg8::A))),
            0xCE => Some(Instruction::Adc(Target::Value)),

            0x90 => Some(Instruction::Sub(Target::Reg8(Reg8::B))),
            0x91 => Some(Instruction::Sub(Target::Reg8(Reg8::C))),
            0x92 => Some(Instruction::Sub(Target::Reg8(Reg8::D))),
            0x93 => Some(Instruction::Sub(Target::Reg8(Reg8::E))),
            0x94 => Some(Instruction::Sub(Target::Reg8(Reg8::H))),
            0x95 => Some(Instruction::Sub(Target::Reg8(Reg8::L))),
            0x96 => Some(Instruction::Sub(Target::Reg16Indirect(Reg16::HL))),
            0x97 => Some(Instruction::Sub(Target::Reg8(Reg8::A))),
            0xD6 => Some(Instruction::Sub(Target::Value)),

            0x98 => Some(Instruction::Sbc(Target::Reg8(Reg8::B))),
            0x99 => Some(Instruction::Sbc(Target::Reg8(Reg8::C))),
            0x9A => Some(Instruction::Sbc(Target::Reg8(Reg8::D))),
            0x9B => Some(Instruction::Sbc(Target::Reg8(Reg8::E))),
            0x9C => Some(Instruction::Sbc(Target::Reg8(Reg8::H))),
            0x9D => Some(Instruction::Sbc(Target::Reg8(Reg8::L))),
            0x9E => Some(Instruction::Sbc(Target::Reg16Indirect(Reg16::HL))),
            0x9F => Some(Instruction::Sbc(Target::Reg8(Reg8::A))),
            0xDE => Some(Instruction::Sbc(Target::Value)),

            0xA0 => Some(Instruction::And(Target::Reg8(Reg8::B))),
            0xA1 => Some(Instruction::And(Target::Reg8(Reg8::C))),
            0xA2 => Some(Instruction::And(Target::Reg8(Reg8::D))),
            0xA3 => Some(Instruction::And(Target::Reg8(Reg8::E))),
            0xA4 => Some(Instruction::And(Target::Reg8(Reg8::H))),
            0xA5 => Some(Instruction::And(Target::Reg8(Reg8::L))),
            0xA6 => Some(Instruction::And(Target::Reg16Indirect(Reg16::HL))),
            0xA7 => Some(Instruction::And(Target::Reg8(Reg8::A))),
            0xE6 => Some(Instruction::And(Target::Value)),

            0xA8 => Some(Instruction::Xor(Target::Reg8(Reg8::B))),
            0xA9 => Some(Instruction::Xor(Target::Reg8(Reg8::C))),
            0xAA => Some(Instruction::Xor(Target::Reg8(Reg8::D))),
            0xAB => Some(Instruction::Xor(Target::Reg8(Reg8::E))),
            0xAC => Some(Instruction::Xor(Target::Reg8(Reg8::H))),
            0xAD => Some(Instruction::Xor(Target::Reg8(Reg8::L))),
            0xAE => Some(Instruction::Xor(Target::Reg16Indirect(Reg16::HL))),
            0xAF => Some(Instruction::Xor(Target::Reg8(Reg8::A))),
            0xEE => Some(Instruction::Xor(Target::Value)),

            0xB0 => Some(Instruction::Or(Target::Reg8(Reg8::B))),
            0xB1 => Some(Instruction::Or(Target::Reg8(Reg8::C))),
            0xB2 => Some(Instruction::Or(Target::Reg8(Reg8::D))),
            0xB3 => Some(Instruction::Or(Target::Reg8(Reg8::E))),
            0xB4 => Some(Instruction::Or(Target::Reg8(Reg8::H))),
            0xB5 => Some(Instruction::Or(Target::Reg8(Reg8::L))),
            0xB6 => Some(Instruction::Or(Target::Reg16Indirect(Reg16::HL))),
            0xB7 => Some(Instruction::Or(Target::Reg8(Reg8::A))),
            0xF6 => Some(Instruction::Or(Target::Value)),

            0xB8 => Some(Instruction::Cp(Target::Reg8(Reg8::B))),
            0xB9 => Some(Instruction::Cp(Target::Reg8(Reg8::C))),
            0xBA => Some(Instruction::Cp(Target::Reg8(Reg8::D))),
            0xBB => Some(Instruction::Cp(Target::Reg8(Reg8::E))),
            0xBC => Some(Instruction::Cp(Target::Reg8(Reg8::H))),
            0xBD => Some(Instruction::Cp(Target::Reg8(Reg8::L))),
            0xBE => Some(Instruction::Cp(Target::Reg16Indirect(Reg16::HL))),
            0xBF => Some(Instruction::Cp(Target::Reg8(Reg8::A))),
            0xFE => Some(Instruction::Cp(Target::Value)),

            0xC2 => Some(Instruction::JP(JumpTest::NotZero)),
            0xC3 => Some(Instruction::JP(JumpTest::Always)),
            0xD2 => Some(Instruction::JP(JumpTest::NotCarry)),
            0xCA => Some(Instruction::JP(JumpTest::Zero)),
            0xDA => Some(Instruction::JP(JumpTest::Carry)),
            0xE9 => Some(Instruction::JPHL),

            0x20 => Some(Instruction::JR(JumpTest::NotZero)),
            0x18 => Some(Instruction::JR(JumpTest::Always)),
            0x30 => Some(Instruction::JR(JumpTest::NotCarry)),
            0x28 => Some(Instruction::JR(JumpTest::Zero)),
            0x38 => Some(Instruction::JR(JumpTest::Carry)),

            0x01 => Some(Instruction::LD(LoadType::Word(Reg16::BC,Reg16::D16))),
            0x11 => Some(Instruction::LD(LoadType::Word(Reg16::DE,Reg16::D16))),
            0x21 => Some(Instruction::LD(LoadType::Word(Reg16::HL,Reg16::D16))),
            0x31 => Some(Instruction::LD(LoadType::Word(Reg16::SP,Reg16::D16))),

            0x40 => Some(Instruction::LD(LoadType::Byte(Reg8::B,Reg8::B))),
            0x41 => Some(Instruction::LD(LoadType::Byte(Reg8::B,Reg8::C))),
            0x42 => Some(Instruction::LD(LoadType::Byte(Reg8::B,Reg8::D))),
            0x43 => Some(Instruction::LD(LoadType::Byte(Reg8::B,Reg8::E))),
            0x44 => Some(Instruction::LD(LoadType::Byte(Reg8::B,Reg8::H))),
            0x45 => Some(Instruction::LD(LoadType::Byte(Reg8::B,Reg8::L))),
            0x46 => Some(Instruction::LD(LoadType::Byte(Reg8::B,Reg8::HLI))),
            0x47 => Some(Instruction::LD(LoadType::Byte(Reg8::B,Reg8::A))),
            0x48 => Some(Instruction::LD(LoadType::Byte(Reg8::C,Reg8::B))),
            0x49 => Some(Instruction::LD(LoadType::Byte(Reg8::C,Reg8::C))),
            0x4A => Some(Instruction::LD(LoadType::Byte(Reg8::C,Reg8::D))),
            0x4B => Some(Instruction::LD(LoadType::Byte(Reg8::C,Reg8::E))),
            0x4C => Some(Instruction::LD(LoadType::Byte(Reg8::C,Reg8::H))),
            0x4D => Some(Instruction::LD(LoadType::Byte(Reg8::C,Reg8::L))),
            0x4E => Some(Instruction::LD(LoadType::Byte(Reg8::C,Reg8::HLI))),
            0x4F => Some(Instruction::LD(LoadType::Byte(Reg8::C,Reg8::A))),

            0x50 => Some(Instruction::LD(LoadType::Byte(Reg8::D,Reg8::B))),
            0x51 => Some(Instruction::LD(LoadType::Byte(Reg8::D,Reg8::C))),
            0x52 => Some(Instruction::LD(LoadType::Byte(Reg8::D,Reg8::D))),
            0x53 => Some(Instruction::LD(LoadType::Byte(Reg8::D,Reg8::E))),
            0x54 => Some(Instruction::LD(LoadType::Byte(Reg8::D,Reg8::H))),
            0x55 => Some(Instruction::LD(LoadType::Byte(Reg8::D,Reg8::L))),
            0x56 => Some(Instruction::LD(LoadType::Byte(Reg8::D,Reg8::HLI))),
            0x57 => Some(Instruction::LD(LoadType::Byte(Reg8::D,Reg8::A))),
            0x58 => Some(Instruction::LD(LoadType::Byte(Reg8::E,Reg8::B))),
            0x59 => Some(Instruction::LD(LoadType::Byte(Reg8::E,Reg8::C))),
            0x5A => Some(Instruction::LD(LoadType::Byte(Reg8::E,Reg8::D))),
            0x5B => Some(Instruction::LD(LoadType::Byte(Reg8::E,Reg8::E))),
            0x5C => Some(Instruction::LD(LoadType::Byte(Reg8::E,Reg8::H))),
            0x5D => Some(Instruction::LD(LoadType::Byte(Reg8::E,Reg8::L))),
            0x5E => Some(Instruction::LD(LoadType::Byte(Reg8::E,Reg8::HLI))),
            0x5F => Some(Instruction::LD(LoadType::Byte(Reg8::E,Reg8::A))),

            0x60 => Some(Instruction::LD(LoadType::Byte(Reg8::H,Reg8::B))),
            0x61 => Some(Instruction::LD(LoadType::Byte(Reg8::H,Reg8::C))),
            0x62 => Some(Instruction::LD(LoadType::Byte(Reg8::H,Reg8::D))),
            0x63 => Some(Instruction::LD(LoadType::Byte(Reg8::H,Reg8::E))),
            0x64 => Some(Instruction::LD(LoadType::Byte(Reg8::H,Reg8::H))),
            0x65 => Some(Instruction::LD(LoadType::Byte(Reg8::H,Reg8::L))),
            0x66 => Some(Instruction::LD(LoadType::Byte(Reg8::H,Reg8::HLI))),
            0x67 => Some(Instruction::LD(LoadType::Byte(Reg8::H,Reg8::A))),
            0x68 => Some(Instruction::LD(LoadType::Byte(Reg8::L,Reg8::B))),
            0x69 => Some(Instruction::LD(LoadType::Byte(Reg8::L,Reg8::C))),
            0x6A => Some(Instruction::LD(LoadType::Byte(Reg8::L,Reg8::D))),
            0x6B => Some(Instruction::LD(LoadType::Byte(Reg8::L,Reg8::E))),
            0x6C => Some(Instruction::LD(LoadType::Byte(Reg8::L,Reg8::H))),
            0x6D => Some(Instruction::LD(LoadType::Byte(Reg8::L,Reg8::L))),
            0x6E => Some(Instruction::LD(LoadType::Byte(Reg8::L,Reg8::HLI))),
            0x6F => Some(Instruction::LD(LoadType::Byte(Reg8::L,Reg8::A))),

            0x70 => Some(Instruction::LD(LoadType::Byte(Reg8::HLI,Reg8::B))),
            0x71 => Some(Instruction::LD(LoadType::Byte(Reg8::HLI,Reg8::C))),
            0x72 => Some(Instruction::LD(LoadType::Byte(Reg8::HLI,Reg8::D))),
            0x73 => Some(Instruction::LD(LoadType::Byte(Reg8::HLI,Reg8::E))),
            0x74 => Some(Instruction::LD(LoadType::Byte(Reg8::HLI,Reg8::H))),
            0x75 => Some(Instruction::LD(LoadType::Byte(Reg8::HLI,Reg8::L))),
            0x76 => Some(Instruction::HALT),
            0x77 => Some(Instruction::LD(LoadType::Byte(Reg8::HLI,Reg8::A))),
            0x78 => Some(Instruction::LD(LoadType::Byte(Reg8::A,Reg8::B))),
            0x79 => Some(Instruction::LD(LoadType::Byte(Reg8::A,Reg8::C))),
            0x7A => Some(Instruction::LD(LoadType::Byte(Reg8::A,Reg8::D))),
            0x7B => Some(Instruction::LD(LoadType::Byte(Reg8::A,Reg8::E))),
            0x7C => Some(Instruction::LD(LoadType::Byte(Reg8::A,Reg8::H))),
            0x7D => Some(Instruction::LD(LoadType::Byte(Reg8::A,Reg8::L))),
            0x7E => Some(Instruction::LD(LoadType::Byte(Reg8::A,Reg8::HLI))),
            0x7F => Some(Instruction::LD(LoadType::Byte(Reg8::A,Reg8::A))),
            
            0x06 => Some(Instruction::LD(LoadType::Byte(Reg8::B,Reg8::D8))),
            0x16 => Some(Instruction::LD(LoadType::Byte(Reg8::D,Reg8::D8))),
            0x26 => Some(Instruction::LD(LoadType::Byte(Reg8::H,Reg8::D8))),
            0x36 => Some(Instruction::LD(LoadType::Byte(Reg8::HLI,Reg8::D8))),
            0x0E => Some(Instruction::LD(LoadType::Byte(Reg8::C,Reg8::D8))),
            0x1E => Some(Instruction::LD(LoadType::Byte(Reg8::E,Reg8::D8))),
            0x2E => Some(Instruction::LD(LoadType::Byte(Reg8::L,Reg8::D8))),
            0x3E => Some(Instruction::LD(LoadType::Byte(Reg8::A,Reg8::D8))),

            0x02 => Some(Instruction::LD(LoadType::Byte(Reg8::BCI,Reg8::A))),
            0x12 => Some(Instruction::LD(LoadType::Byte(Reg8::DEI,Reg8::A))),
            0x22 => Some(Instruction::LD(LoadType::Byte(Reg8::HLII,Reg8::A))),
            0x32 => Some(Instruction::LD(LoadType::Byte(Reg8::HLDI,Reg8::A))),
            0x0A => Some(Instruction::LD(LoadType::Byte(Reg8::A,Reg8::BCI))),
            0x1A => Some(Instruction::LD(LoadType::Byte(Reg8::A,Reg8::DEI))),
            0x2A => Some(Instruction::LD(LoadType::Byte(Reg8::A,Reg8::HLII))),
            0x3A => Some(Instruction::LD(LoadType::Byte(Reg8::A,Reg8::HLDI))),

            0xEA => Some(Instruction::LD(LoadType::Byte(Reg8::D16I,Reg8::A))),
            0xFA => Some(Instruction::LD(LoadType::Byte(Reg8::A,Reg8::D16I))),

            0xE0 => Some(Instruction::LD(LoadType::Byte(Reg8::D8I,Reg8::A))),
            0xE2 => Some(Instruction::LD(LoadType::Byte(Reg8::CI,Reg8::A))),
            0xF0 => Some(Instruction::LD(LoadType::Byte(Reg8::A,Reg8::D8I))),
            0xF2 => Some(Instruction::LD(LoadType::Byte(Reg8::A,Reg8::CI))),

            0xF3 => Some(Instruction::DI),

            0xC1 => Some(Instruction::POP(StackTarget::BC)),
            0xD1 => Some(Instruction::POP(StackTarget::DE)),
            0xE1 => Some(Instruction::POP(StackTarget::HL)),
            0xF1 => Some(Instruction::POP(StackTarget::AF)),

            0xC5 => Some(Instruction::PUSH(StackTarget::BC)),
            0xD5 => Some(Instruction::PUSH(StackTarget::DE)),
            0xE5 => Some(Instruction::PUSH(StackTarget::HL)),
            0xF5 => Some(Instruction::PUSH(StackTarget::AF)),

            0xC4 => Some(Instruction::CALL(JumpTest::NotZero)),
            0xCD => Some(Instruction::CALL(JumpTest::Always)),
            0xD4 => Some(Instruction::CALL(JumpTest::NotCarry)),
            0xCC => Some(Instruction::CALL(JumpTest::Zero)),
            0xDC => Some(Instruction::CALL(JumpTest::Carry)),

            0xC0 => Some(Instruction::RET(JumpTest::NotZero)),
            0xC9 => Some(Instruction::RET(JumpTest::Always)),
            0xD0 => Some(Instruction::RET(JumpTest::NotCarry)),
            0xC8 => Some(Instruction::RET(JumpTest::Zero)),
            0xD8 => Some(Instruction::RET(JumpTest::Carry)),

            _ => { /* Add more instructions */ None }
        }
    }
}

pub struct CPU {
    pub registers: reg::Registers,
    sp: u16,
    pub pc: u16,
    bus: memory::MemoryBus,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: reg::Registers {
                a: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                f: reg::FlagsRegister {
                    zero: false,
                    subtract: false,
                    half_carry: false,
                    carry: false
                },
                h: 0,
                l: 0,
            },
            sp: 0,
            pc: 0,
            bus: memory::MemoryBus {
                memory: [0; 0x10000]
            }
        }
    }

    pub fn load_rom(&mut self, filepath: &str) {
        let rom: Vec<u8> = std::fs::read(filepath).unwrap();
        self.bus.memory[0..32768].copy_from_slice(&rom[..]);
        self.pc = 0x100;
    }

    pub fn run_sm83_tests(&mut self, op_codes: &Vec<u8>, prefixed: bool) {
        for &i in op_codes {
            let tests: Vec<CpuTest> = serde_json::from_str::<Vec<CpuTest>>(
                    &String::from_utf8(
                        std::fs::read(
                            format!("sm83/v1/{}{i:02x}.json", if prefixed { "cb " } else { "" })
                        ).unwrap()
                    ).unwrap()
                ).unwrap();
            for test in tests {
                self.set_state(&test.initial_state);
                let instruction: Instruction = Instruction::from_byte(i, prefixed).unwrap();
                self.pc = self.execute(instruction);
                self.compare_state(&test.final_state);
            }
            println!("0x{}{i:02x} passed!", if prefixed { "cb" } else { "" });
        }
    }

    pub fn set_state(&mut self, state: &CpuState) {
        self.registers.a = state.a;
        self.registers.b = state.b;
        self.registers.c = state.c;
        self.registers.d = state.d;
        self.registers.e = state.e;
        self.registers.f = FlagsRegister::from(state.f);
        self.registers.h = state.h;
        self.registers.l = state.l;

        self.sp = state.sp;
        self.pc = state.pc;

        for m in state.ram.clone() {
            self.bus.memory[m[0] as usize] = m[1] as u8;
        }
    }

    pub fn compare_state(&self, state: &CpuState) {
        /* Compare registers */
        assert_eq!(self.registers.a, state.a, "Register A: {} (expected {})", self.registers.a, state.a);
        assert_eq!(self.registers.b, state.b, "Register B: {} (expected {})", self.registers.b, state.b);
        assert_eq!(self.registers.c, state.c, "Register C: {} (expected {})", self.registers.c, state.c);
        assert_eq!(self.registers.d, state.d, "Register D: {} (expected {})", self.registers.d, state.d);
        assert_eq!(self.registers.e, state.e, "Register E: {} (expected {})", self.registers.e, state.e);
        assert_eq!(u8::from(self.registers.f), state.f, "Flags: {} (expected {})", u8::from(self.registers.f), state.f);
        assert_eq!(self.registers.h, state.h, "Register H: {} (expected {})", self.registers.h, state.h);
        assert_eq!(self.registers.l, state.l, "Register L: {} (expected {})", self.registers.l, state.l);

        /* Compare PC and SP */
        assert_eq!(self.pc, state.pc, "PC: {} (expected {})", self.pc, state.pc);
        assert_eq!(self.sp, state.sp, "SP: {} (expected {})", self.sp, state.sp);

        /* Compare memory */
        for r in state.ram.clone() {
            assert_eq!(self.bus.memory[r[0] as usize], r[1] as u8, "RAM at addr: {}: {} (expected {})", r[0], 
                self.bus.memory[r[0] as usize], r[1]);
        }
    }

    pub fn run(&mut self) {
        loop {
            self.step();

            if (self.bus.memory[0xff02] == 0x81) {
                print!("{}", self.bus.memory[0xff01] as char);
                self.bus.memory[0xff01] = 0;
            }
        }
    }

    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;

        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc.wrapping_add(1));
        }

        let next_pc: u16 = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        } else {
            println!("{:?} PC:{} SP:{} RAM at PC:{}", self.registers, self.pc, self.sp, self.bus.memory[self.pc as usize]);
            panic!("Unknown instruction: 0x{instruction_byte:x}");
        };

        self.pc = next_pc;
    }

    // executes an instruction decoded by the step() method
    pub fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::JP(test) => {
                let jump_condition: bool = match test {
                    JumpTest::Always => true,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::NotZero => !self.registers.f.zero
                };
                self.jp(jump_condition)
            }
            Instruction::JR(test) => {
                let jump_condition: bool = match test {
                    JumpTest::Always => true,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::NotZero => !self.registers.f.zero
                };
                self.jr(jump_condition)
            }
            Instruction::JPHL => self.jphl(),
            Instruction::DI => { self.pc.wrapping_add(1) },

            Instruction::LD(load_type) => self.ld(load_type),

            Instruction::CALL(test) => {
                let jump_condition: bool = match test {
                    JumpTest::Always => true,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::NotZero => !self.registers.f.zero
                };
                self.call(jump_condition)
            }

            Instruction::RET(test) => {
                let jump_condition: bool = match test {
                    JumpTest::Always => true,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::NotZero => !self.registers.f.zero
                };
                self.ret(jump_condition)
            }

            Instruction::PUSH(target) => {
                let value = match target {
                    StackTarget::AF => {
                        self.registers.get_af()
                    }
                    StackTarget::BC => {
                        self.registers.get_bc()
                    }
                    StackTarget::DE => {
                        self.registers.get_de()
                    }
                    StackTarget::HL => {
                        self.registers.get_hl()
                    }
                };
                self.push(value)
            }

            Instruction::POP(target) => {
                let (pc, value) = self.pop();
                match target {
                    StackTarget::AF => {
                        self.registers.set_af(value);
                    }
                    StackTarget::BC => {
                        self.registers.set_bc(value);
                    }
                    StackTarget::DE => {
                        self.registers.set_de(value);
                    }
                    StackTarget::HL => {
                        self.registers.set_hl(value);
                    }
                };
                pc
            }

            Instruction::Nop => self.nop(),
            Instruction::Cpl => self.cpl(),
            Instruction::Ccf => self.ccf(),
            Instruction::Scf => self.scf(),
            Instruction::Rlca => self.rlca(),
            Instruction::Rla => self.rla(),
            Instruction::Rrca => self.rrca(),
            Instruction::Rra => self.rra(),
            Instruction::HALT => { 
                println!("halt");
                self.pc.wrapping_add(1)
            },
            Instruction::Bit0(target) => self.bit(target, 0),
            Instruction::Bit1(target) => self.bit(target, 1),
            Instruction::Bit2(target) => self.bit(target, 2),
            Instruction::Bit3(target) => self.bit(target, 3),
            Instruction::Bit4(target) => self.bit(target, 4),
            Instruction::Bit5(target) => self.bit(target, 5),
            Instruction::Bit6(target) => self.bit(target, 6),
            Instruction::Bit7(target) => self.bit(target, 7),
            Instruction::Res0(target) => self.res(target, 0),
            Instruction::Res1(target) => self.res(target, 1),
            Instruction::Res2(target) => self.res(target, 2),
            Instruction::Res3(target) => self.res(target, 3),
            Instruction::Res4(target) => self.res(target, 4),
            Instruction::Res5(target) => self.res(target, 5),
            Instruction::Res6(target) => self.res(target, 6),
            Instruction::Res7(target) => self.res(target, 7),
            Instruction::Set0(target) => self.set(target, 0),
            Instruction::Set1(target) => self.set(target, 1),
            Instruction::Set2(target) => self.set(target, 2),
            Instruction::Set3(target) => self.set(target, 3),
            Instruction::Set4(target) => self.set(target, 4),
            Instruction::Set5(target) => self.set(target, 5),
            Instruction::Set6(target) => self.set(target, 6),
            Instruction::Set7(target) => self.set(target, 7),
            Instruction::Srl(target) => self.srl(target),
            Instruction::Swap(target) => self.swap(target),
            Instruction::Sra(target) => self.sra(target),
            Instruction::Sla(target) => self.sla(target),
            Instruction::Rr(target) => self.rr(target),
            Instruction::Rl(target) => self.rl(target),
            Instruction::Rrc(target) => self.rrc(target),
            Instruction::Rlc(target) => self.rlc(target),
            Instruction::Inc(target) => self.inc(target),
            Instruction::Dec(target) => self.dec(target),
            Instruction::Add(target) => self.add(target),
            Instruction::AddHL(target) => self.addhl(target),
            Instruction::Adc(target) => self.adc(target),
            Instruction::Sub(target) => self.sub(target),
            Instruction::Sbc(target) => self.sbc(target),
            Instruction::Or(target) => self.or(target),
            Instruction::And(target) => self.and(target),
            Instruction::Xor(target) => self.xor(target),
            Instruction::Cp(target) => self.cp(target),
            _ => { /* Add more instructions */ self.pc }
        }

    }

    fn reg8_lookup(&mut self, register: Reg8) -> &mut u8 {
        match register {
            Reg8::B => &mut self.registers.b,
            Reg8::C => &mut self.registers.c,
            Reg8::D => &mut self.registers.d,
            Reg8::E => &mut self.registers.e,
            Reg8::H => &mut self.registers.h,
            Reg8::L => &mut self.registers.l,
            Reg8::A => &mut self.registers.a,
            _ => { panic!("D8 lookup"); }
        }
    }

    fn reg16_lookup(&self, register: Reg16) -> u16 {
        match register {
            Reg16::AF => self.registers.get_af(),
            Reg16::BC => self.registers.get_bc(),
            Reg16::DE => self.registers.get_de(),
            Reg16::HL => self.registers.get_hl(),
            _ => 0
        }
    }

    fn ref_from_target(&mut self, target: Target) -> Option<&mut u8> {
        match target {
            Target::Reg8(r) => {
                Some(self.reg8_lookup(r))
            }
            Target::Reg16Indirect(r) => {
                let addr: u16 = self.reg16_lookup(r);
                Some(self.bus.get_ref(addr))
            }
            Target::Value => {
                Some(&mut self.bus.memory[self.pc.wrapping_add(1) as usize])
            }
            _ => { None }
        }
    }

    fn call(&mut self, jump: bool) -> u16 {
        let pc_next = self.pc.wrapping_add(3);
        if jump {
            self.push(pc_next);
            let lsb: u16 = self.bus.read_byte(self.pc.wrapping_add(1)) as u16;
            let msb: u16 = self.bus.read_byte(self.pc.wrapping_add(2)) as u16;
            (msb << 8) | lsb
        } else {
            pc_next
        }
    }

    fn ret(&mut self, jump: bool) -> u16 {
        if jump {
            self.pop().1
        } else {
            self.pc.wrapping_add(1)
        }
    }

    fn push(&mut self, value: u16) -> u16 {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8);

        self.pc.wrapping_add(1)
    }

    fn pop(&mut self) -> (u16,u16) {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (self.pc.wrapping_add(1), (msb << 8) | lsb)
    }

    fn ld(&mut self, load_type: LoadType) -> u16 {
        match load_type {
            LoadType::Word(target, source) => {
                let source_value: u16 = match source {
                    Reg16::D16 => {
                        let lsb: u16 = self.bus.read_byte(self.pc.wrapping_add(1)) as u16;
                        let msb: u16 = self.bus.read_byte(self.pc.wrapping_add(2)) as u16;
                        (msb << 8) | lsb
                    }
                    _ => { panic!("Unknown load source!"); }
                };
                match target {
                    Reg16::BC => {
                        self.registers.set_bc(source_value);
                    }
                    Reg16::DE => {
                        self.registers.set_de(source_value);
                    }
                    Reg16::HL => {
                        self.registers.set_hl(source_value);
                    }
                    Reg16::SP => {
                        self.sp = source_value;
                    }
                    _ => { panic!("Unknown load target!"); }
                }
                self.pc.wrapping_add(3)
            }
            LoadType::Byte(target,source) => {
                let source_value: u8 = match source {
                    Reg8::A => self.registers.a,
                    Reg8::B => self.registers.b,
                    Reg8::C => self.registers.c,
                    Reg8::D => self.registers.d,
                    Reg8::E => self.registers.e,
                    Reg8::H => self.registers.h,
                    Reg8::L => self.registers.l,
                    Reg8::D8 => self.bus.read_byte(self.pc.wrapping_add(1)),
                    Reg8::BCI => self.bus.read_byte(self.registers.get_bc()),
                    Reg8::DEI => self.bus.read_byte(self.registers.get_de()),
                    Reg8::HLI => self.bus.read_byte(self.registers.get_hl()),
                    Reg8::HLII => {
                        let hl = self.registers.get_hl();
                        let value = self.bus.read_byte(hl);
                        self.registers.set_hl(hl.wrapping_add(1));
                        value
                    }
                    Reg8::HLDI => {
                        let hl = self.registers.get_hl();
                        let value = self.bus.read_byte(hl);
                        self.registers.set_hl(hl.wrapping_sub(1));
                        value
                    }
                    Reg8::D16I => {
                        let lsb: u16 = self.bus.read_byte(self.pc.wrapping_add(1)) as u16;
                        let msb: u16 = self.bus.read_byte(self.pc.wrapping_add(2)) as u16;
                        let addr = (msb << 8) | lsb;
                        self.bus.read_byte(addr)
                    }
                    Reg8::D8I => {
                        let byte: u16 = self.bus.read_byte(self.pc.wrapping_add(1)) as u16;
                        let addr: u16 = 0xFF00 + byte;
                        self.bus.read_byte(addr)
                    }
                    Reg8::CI => {
                        let addr = 0xFF00_u16 + (self.registers.c as u16);
                        self.bus.read_byte(addr)
                    }
                };
                match target {
                    Reg8::A => { self.registers.a = source_value; }
                    Reg8::B => { self.registers.b = source_value; }
                    Reg8::C => { self.registers.c = source_value; }
                    Reg8::D => { self.registers.d = source_value; }
                    Reg8::E => { self.registers.e = source_value; }
                    Reg8::H => { self.registers.h = source_value; }
                    Reg8::L => { self.registers.l = source_value; }
                    Reg8::BCI => { self.bus.write_byte(self.registers.get_bc(), source_value); }
                    Reg8::DEI => { self.bus.write_byte(self.registers.get_de(), source_value); }
                    Reg8::HLI => { self.bus.write_byte(self.registers.get_hl(), source_value); }
                    Reg8::HLII => {
                        let hl = self.registers.get_hl();
                        self.bus.write_byte(hl, source_value);
                        self.registers.set_hl(hl.wrapping_add(1));
                        
                    }
                    Reg8::HLDI => {
                        let hl = self.registers.get_hl();
                        self.bus.write_byte(hl, source_value);
                        self.registers.set_hl(hl.wrapping_sub(1));
                    }
                    Reg8::D16I => {
                        let lsb: u16 = self.bus.read_byte(self.pc.wrapping_add(1)) as u16;
                        let msb: u16 = self.bus.read_byte(self.pc.wrapping_add(2)) as u16;
                        let addr = (msb << 8) | lsb;
                        self.bus.write_byte(addr, source_value);
                    }
                    Reg8::D8I => {
                        let byte: u16 = self.bus.read_byte(self.pc.wrapping_add(1)) as u16;
                        let addr: u16 = 0xFF00 + byte;
                        self.bus.write_byte(addr, source_value);
                    }
                    Reg8::CI => {
                        let addr = 0xFF00_u16 + (self.registers.c as u16);
                        self.bus.write_byte(addr, source_value);
                    }
                    _ => { panic!("D8 Target"); }
                }
                match (target,source) {
                    (_,Reg8::D8)   => self.pc.wrapping_add(2),
                    (Reg8::D8I,_)  => self.pc.wrapping_add(2),
                    (_,Reg8::D8I)  => self.pc.wrapping_add(2),

                    (Reg8::D16I,_) => self.pc.wrapping_add(3),
                    (_,Reg8::D16I) => self.pc.wrapping_add(3),

                    (_,_)          => self.pc.wrapping_add(1)
                }
                
            }
        }
    }

    fn jp(&self, jump: bool) -> u16 {
        if jump {
            let lsb: u16 = self.bus.read_byte(self.pc.wrapping_add(1)) as u16;
            let msb: u16 = self.bus.read_byte(self.pc.wrapping_add(2)) as u16;
            (msb << 8) | lsb
        } else {
            self.pc.wrapping_add(3)
        }
    }
    
    fn jr(&self, jump: bool) -> u16 {
        if jump {
            let rel_addr: i8 = self.bus.read_byte(self.pc.wrapping_add(1)) as i8;
            let mag: u16 = rel_addr.unsigned_abs() as u16;

            if (rel_addr & -128) == -128 {
                self.pc.wrapping_add(2).wrapping_sub(mag)
            } else { 
                self.pc.wrapping_add(2).wrapping_add(mag)
            }

        } else {
            self.pc.wrapping_add(2)
        }
    }

    fn jphl(&self) -> u16 {
        self.registers.get_hl()
    }

    fn addhl(&mut self, target: AddHLTarget) -> u16 {

        let source_value = match target {
            AddHLTarget::HL => self.registers.get_hl(),
            AddHLTarget::BC => self.registers.get_bc(),
            AddHLTarget::DE => self.registers.get_de(),
            AddHLTarget::SP => self.sp
        };
        
        let (result, did_overflow) = self.registers.get_hl().overflowing_add(source_value);
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (self.registers.get_hl() & 0xFFF) + (source_value & 0xFFF) > 0xFFF;
        self.registers.f.carry = did_overflow;

        self.registers.set_hl(result);

        self.pc.wrapping_add(1)
    }
    
    fn add(&mut self, target: Target) -> u16 {

        let pc_update: u16 = match target {
            Target::Value => 2,
            _ => 1
        };

        let byte = {
            *self.ref_from_target(target).unwrap()
        };
        
        let (result, did_overflow) = self.registers.a.overflowing_add(byte);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (byte & 0xF) > 0xF;
        
        self.registers.a = result;

        self.pc.wrapping_add(pc_update)
    }

    fn adc(&mut self, target: Target) -> u16 {
        let pc_update: u16 = match target {
            Target::Value => 2,
            _ => 1
        };

        let byte = {
            *self.ref_from_target(target).unwrap()
        };

        let carry = if self.registers.f.carry { 1 } else { 0 };

        let (result, overflow1) = self.registers.a.overflowing_add(byte);
        let (result,  overflow2) = result.overflowing_add(carry);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = overflow1 || overflow2;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (byte & 0xF) + carry > 0xF;

        self.registers.a = result;

        self.pc.wrapping_add(pc_update)
    }

    fn sub(&mut self, target: Target) -> u16 {
        let pc_update: u16 = match target {
            Target::Value => 2,
            _ => 1
        };

        let byte = {
            *self.ref_from_target(target).unwrap()
        };

        let (result, did_overflow) = self.registers.a.overflowing_sub(byte);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = ((self.registers.a & 0xF) as i8) - ((byte & 0xF) as i8) < 0;

        self.registers.a = result;

        self.pc.wrapping_add(pc_update)
    }

    fn sbc(&mut self, target: Target) -> u16 {
        let pc_update: u16 = match target {
            Target::Value => 2,
            _ => 1
        };

        let byte = {
            *self.ref_from_target(target).unwrap()
        };

        let carry = if self.registers.f.carry { 1 } else { 0 };
        let (result, overflow1) = self.registers.a.overflowing_sub(byte);
        let (result, overflow2) = result.overflowing_sub(carry);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = overflow1 || overflow2;
        self.registers.f.half_carry = ((self.registers.a & 0xF) as i8) - ((byte & 0xF) as i8) - (carry as i8) < 0;

        self.registers.a = result;

        self.pc.wrapping_add(pc_update)
    }

    fn or(&mut self, target: Target) -> u16 {
        let pc_update: u16 = match target {
            Target::Value => 2,
            _ => 1
        };

        let byte = {
            *self.ref_from_target(target).unwrap()
        };

        let result = self.registers.a | byte;
        
        self.registers.f.clear_all();
        self.registers.f.zero = result == 0;

        self.registers.a = result;

        self.pc.wrapping_add(pc_update)
    }

    fn and(&mut self, target: Target) -> u16 {
        let pc_update: u16 = match target {
            Target::Value => 2,
            _ => 1
        };

        let byte = {
            *self.ref_from_target(target).unwrap()
        };

        let result = self.registers.a & byte;

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = true;

        self.registers.a = result;

        self.pc.wrapping_add(pc_update)
    }

    fn xor(&mut self, target: Target) -> u16 {
        let pc_update: u16 = match target {
            Target::Value => 2,
            _ => 1
        };

        let byte = {
            *self.ref_from_target(target).unwrap()
        };

        let result = self.registers.a ^ byte;

        self.registers.f.clear_all();
        self.registers.f.zero = result == 0;

        self.registers.a = result;

        self.pc.wrapping_add(pc_update)
    }

    fn cp(&mut self, target: Target) -> u16 {
        let pc_update: u16 = match target {
            Target::Value => 2,
            _ => 1
        };

        let byte = {
            *self.ref_from_target(target).unwrap()
        };

        let (result, did_overflow) = self.registers.a.overflowing_sub(byte);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = ((self.registers.a & 0xF) as i8) - ((byte & 0xF) as i8) < 0;

        self.pc.wrapping_add(pc_update)
    }

    fn cpl(&mut self) -> u16 {
        self.registers.a = !self.registers.a;

        // z and c unmodified
        self.registers.f.subtract = true;
        self.registers.f.half_carry = true;
        
        self.pc.wrapping_add(1) 
    }

    fn ccf(&mut self) -> u16 {
        // zero flag unmodified
        self.registers.f.subtract = false;
        self.registers.f.carry = !self.registers.f.carry;
        self.registers.f.half_carry = false;

        self.pc.wrapping_add(1)
    }

    fn scf(&mut self) -> u16 {
        // zero flag unmodified
        self.registers.f.subtract = false;
        self.registers.f.carry = true;
        self.registers.f.half_carry = false;

        self.pc.wrapping_add(1)
    }

    fn nop(&self) -> u16 {
        self.pc.wrapping_add(1)
    }

    fn rlca(&mut self) -> u16 {
        let bit7: u8 = if (self.registers.a & 0x80) > 0 {1} else { 0 };
        self.registers.a = self.registers.a.rotate_left(1);

        self.registers.f.clear_all();
        self.registers.f.carry = bit7 != 0;

        self.pc.wrapping_add(1)
    }

    fn rla(&mut self) -> u16 {
        let carry_bit: u8 = if self.registers.f.carry { 1 } else { 0 };
        let bit7: u8 = if (self.registers.a & 0x80) > 0 {1} else { 0 };

        self.registers.a <<= 1;
        self.registers.a |= carry_bit;

        self.registers.f.clear_all();
        self.registers.f.carry = bit7 != 0;

        self.pc.wrapping_add(1)
    }

    fn rrca(&mut self) -> u16 {
        let bit0 = self.registers.a & 0x1;
        self.registers.a = self.registers.a.rotate_right(1);

        self.registers.f.clear_all();
        self.registers.f.carry = bit0 != 0;

        self.pc.wrapping_add(1)
    }

    fn rra(&mut self) -> u16 {
        let bit0 = self.registers.a & 0x1;
        let carry_bit: u8 = if self.registers.f.carry { 1 } else { 0 };
        self.registers.a >>= 1;
        self.registers.a |= (carry_bit << 7);

        self.registers.f.clear_all();
        self.registers.f.carry = bit0 != 0;

        self.pc.wrapping_add(1)
    }

    pub fn inc(&mut self, target: Target) -> u16 {
        match target {
            Target::Reg16(t) => {
                match t {
                    Reg16::BC => {
                        let value = self.registers.get_bc();
                        self.registers.set_bc(value.wrapping_add(1));
                    }
                    Reg16::DE => {
                        let value = self.registers.get_de();
                        self.registers.set_de(value.wrapping_add(1));
                    }
                    Reg16::HL => {
                        let value = self.registers.get_hl();
                        self.registers.set_hl(value.wrapping_add(1));
                    }
                    Reg16::SP => {
                        self.sp = self.sp.wrapping_add(1);
                    }
                    _ => { panic!("INC unknown reg16 target"); }
                }
            }
            _ => {
                let byte = self.ref_from_target(target).unwrap();
                let prior = *byte;
                *byte = byte.wrapping_add(1);

                self.registers.f.zero = *byte == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (prior & 0xF) + 1 > 0xF;
            }
        }
        
        self.pc.wrapping_add(1)
    }

    fn dec(&mut self, target: Target) -> u16 {
        let byte_ref: Option<&mut u8> = self.ref_from_target(target);

        if let Some(byte) = byte_ref {
            let prior = *byte;
            *byte = byte.wrapping_sub(1);

            self.registers.f.zero = *byte == 0;
            self.registers.f.half_carry = ((prior & 0xF) as i8) - 1_i8 < 0;
            self.registers.f.subtract = true;
            
            self.pc.wrapping_add(1)
        } else {
            panic!("DEC unknown target");
        }
    }

    fn rlc(&mut self, target: Target) -> u16 {
        let carry: u8 = if self.registers.f.carry { 1 } else { 0 };
        let byte_ref: Option<&mut u8> = self.ref_from_target(target);

        if let Some(byte) = byte_ref {
            let bit7: u8 = if (*byte & 0x80) > 0 { 1 } else { 0 };
            *byte = byte.rotate_left(1);

            self.registers.f.zero = *byte == 0;
            self.registers.f.subtract = false;
            self.registers.f.half_carry = false;
            self.registers.f.carry = bit7 != 0;

            self.pc.wrapping_add(2)
        } else {
            panic!("RLC unknown target");
        }
    }

    fn rrc(&mut self, target: Target) -> u16 {
        let carry: u8 = if self.registers.f.carry { 1 } else { 0 };
        let byte_ref: Option<&mut u8> = self.ref_from_target(target);

        if let Some(byte) = byte_ref {
            let bit0: u8 = *byte & 0x1;
            *byte = byte.rotate_right(1);

            self.registers.f.zero = *byte == 0;
            self.registers.f.subtract = false;
            self.registers.f.half_carry = false;
            self.registers.f.carry = bit0 != 0;

            self.pc.wrapping_add(2)
        } else {
            panic!("RRC unknown target");
        }
    }

    fn rl(&mut self, target: Target) -> u16 {
        let carry: u8 = if self.registers.f.carry { 1 } else { 0 };
        let byte_ref: Option<&mut u8> = self.ref_from_target(target);

        if let Some(byte) = byte_ref {
            let bit7: u8 = if (*byte & 0x80) > 0 { 1 } else { 0 };
            *byte <<= 1;
            *byte |= carry;

            self.registers.f.zero = *byte == 0;
            self.registers.f.subtract = false;
            self.registers.f.half_carry = false;
            self.registers.f.carry = bit7 != 0;

            self.pc.wrapping_add(2)
        } else {
            panic!("RL unknown target");
        }
    }

    fn rr(&mut self, target: Target) -> u16 {
        let carry: u8 = if self.registers.f.carry { 1 } else { 0 };
        let byte_ref: Option<&mut u8> = self.ref_from_target(target);

        if let Some(byte) = byte_ref {
            let bit0: u8 = *byte & 0x1;
            *byte >>= 1;
            *byte |= carry << 7;

            self.registers.f.zero = *byte == 0;
            self.registers.f.subtract = false;
            self.registers.f.half_carry = false;
            self.registers.f.carry = bit0 != 0;

            self.pc.wrapping_add(2)
        } else {
            panic!("RR unknown target");
        }
    }

    fn sla(&mut self, target: Target) -> u16 {
        let byte_ref: Option<&mut u8> = self.ref_from_target(target);

        if let Some(byte) = byte_ref {
            let bit7: u8 = (*byte & 0x80) >> 7;
            *byte <<= 1;

            self.registers.f.zero = *byte == 0;
            self.registers.f.subtract = false;
            self.registers.f.half_carry = false;
            self.registers.f.carry = bit7 != 0;

            self.pc.wrapping_add(2)
        } else {
            panic!("SLA unknown target");
        }
    }

    fn sra(&mut self, target: Target) -> u16 {
        let byte_ref: Option<&mut u8> = self.ref_from_target(target);

        if let Some(byte) = byte_ref {
            let bit7: u8 = *byte & 0x80;
            let bit0: u8 = *byte & 0x1;
            *byte >>= 1;
            *byte |= bit7;

            self.registers.f.zero = *byte == 0;
            self.registers.f.subtract = false;
            self.registers.f.half_carry = false;
            self.registers.f.carry = bit0 != 0;

            self.pc.wrapping_add(2)
        } else {
            panic!("SRA unknown target");
        }
    }

    fn swap(&mut self, target: Target) -> u16 {
        let byte_ref: Option<&mut u8> = self.ref_from_target(target);

        if let Some(byte) = byte_ref {

            *byte = (*byte & 0xF) << 4 | (*byte & 0xF0) >> 4;

            self.registers.f.zero = *byte == 0;
            self.registers.f.subtract = false;
            self.registers.f.half_carry = false;
            self.registers.f.carry = false;
            
            self.pc.wrapping_add(2)
        } else {
            panic!("SWAP unknown target");
        }
    }

    fn srl(&mut self, target: Target) -> u16 {
        let byte_ref: Option<&mut u8> = self.ref_from_target(target);

        if let Some(byte) = byte_ref {
            let bit0: u8 = *byte & 0x1;
            *byte >>= 1;

            self.registers.f.zero = *byte == 0;
            self.registers.f.subtract = false;
            self.registers.f.half_carry = false;
            self.registers.f.carry = bit0 != 0;

            self.pc.wrapping_add(2)
        } else {
            panic!("SRL unknown target");
        }
    }

    fn bit(&mut self, target: Target, bit: u8) -> u16 {
        let byte_ref: Option<&mut u8> = self.ref_from_target(target);
        if let Some(byte) = byte_ref {
            let bit = if bit > 0 { 1 << bit } else { 1 };

            self.registers.f.zero = (*byte & bit) == 0;
            self.registers.f.subtract = false;
            self.registers.f.half_carry = true;

            self.pc.wrapping_add(2)
        } else {
            panic!("BIT{bit} unknown target");
        }
    }

    fn res(&mut self, target: Target, bit: u8) -> u16 {
        let byte_ref: Option<&mut u8> = self.ref_from_target(target);
        if let Some(byte) = byte_ref {
            let bit = if bit > 0 { 1 << bit } else { 1 };
            *byte &= !bit;

            self.pc.wrapping_add(2)
        } else {
            panic!("RES{bit} unknown target");
        }
    }

    fn set(&mut self, target: Target, bit: u8) -> u16 {
        let byte_ref: Option<&mut u8> = self.ref_from_target(target);
        if let Some(byte) = byte_ref {
            let bit = if bit > 0 { 1 << bit } else { 1 };
            *byte |= bit;

            self.pc.wrapping_add(2)
        } else {
            panic!("SET{bit} unknown target");
        }
    }

}