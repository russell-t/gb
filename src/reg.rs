use crate::cpu;

#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister, // <-- flags, [z,n,h,c,0,0,0,0] (lower nibble always 0)
    pub h: u8,
    pub l: u8,
}

#[derive(Debug,Clone,Copy)]
pub struct FlagsRegister {
    pub zero: bool, // z
    pub subtract: bool, // n
    pub half_carry: bool, // h
    pub carry: bool, // c
}

const ZERO_FLAG_BIT_POSITION: u8 = 7;
const SUBTRACT_FLAG_BIT_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BIT_POSITION: u8 = 5;
const CARRY_FLAG_BIT_POSITION: u8 = 4;

impl FlagsRegister {
    pub fn clear_all(&mut self) {
        self.zero = false;
        self.subtract = false;
        self.half_carry = false;
        self.carry = false;
    }
}

impl From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> Self {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BIT_POSITION |
        (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BIT_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BIT_POSITION |
        (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BIT_POSITION
    }
}

impl From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BIT_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BIT_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BIT_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BIT_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}

// 16 bit registers are formed from combining two 8-bit registers
// a 16-bit reg denoted by "xy" means (x << 8) | y
// valid combos are: af, bc, de, hl

impl Registers {
    pub fn get_af(&self) -> u16 {
        let f: u8 = self.f.into();
        (self.a as u16) << 8
        | f as u16
    }
    fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8; 
        self.f = ((value & 0xFF) as u8).into();
    }
    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8
        | (self.c) as u16
    }
    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }
    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8
        | (self.e) as u16
    }
    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }
    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8
        | (self.l) as u16
    }
    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}