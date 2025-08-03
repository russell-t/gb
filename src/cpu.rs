use crate::reg;
use crate::memory;

pub enum Target {
    Reg8(Reg8),
    Reg16(Reg16),
    Reg16Indirect(Reg16),
}

pub enum Reg8 {
    A, B, C, D, E, H, L,
}

// af, bc, de, hl
pub enum Reg16 {
    AF, BC, DE, HL, 
}

pub enum Instruction {
    Add(Target),
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
    Cpl,
    Ccf,
    Scf,
    Nop,
    Rlca,
    Rla,
    Rrca,
    Rra,
}

impl Instruction {
    fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
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

            _ => { /* Add more instructions */ None }
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

            0x88 => Some(Instruction::Adc(Target::Reg8(Reg8::B))),
            0x89 => Some(Instruction::Adc(Target::Reg8(Reg8::C))),
            0x8A => Some(Instruction::Adc(Target::Reg8(Reg8::D))),
            0x8B => Some(Instruction::Adc(Target::Reg8(Reg8::E))),
            0x8C => Some(Instruction::Adc(Target::Reg8(Reg8::H))),
            0x8D => Some(Instruction::Adc(Target::Reg8(Reg8::L))),
            0x8D => Some(Instruction::Adc(Target::Reg16Indirect(Reg16::HL))),
            0x8F => Some(Instruction::Adc(Target::Reg8(Reg8::A))),

            0x90 => Some(Instruction::Sub(Target::Reg8(Reg8::B))),
            0x91 => Some(Instruction::Sub(Target::Reg8(Reg8::C))),
            0x92 => Some(Instruction::Sub(Target::Reg8(Reg8::D))),
            0x93 => Some(Instruction::Sub(Target::Reg8(Reg8::E))),
            0x94 => Some(Instruction::Sub(Target::Reg8(Reg8::H))),
            0x95 => Some(Instruction::Sub(Target::Reg8(Reg8::L))),
            0x96 => Some(Instruction::Sub(Target::Reg16Indirect(Reg16::HL))),
            0x97 => Some(Instruction::Sub(Target::Reg8(Reg8::A))),

            0x98 => Some(Instruction::Sbc(Target::Reg8(Reg8::B))),
            0x99 => Some(Instruction::Sbc(Target::Reg8(Reg8::C))),
            0x9A => Some(Instruction::Sbc(Target::Reg8(Reg8::D))),
            0x9B => Some(Instruction::Sbc(Target::Reg8(Reg8::E))),
            0x9C => Some(Instruction::Sbc(Target::Reg8(Reg8::H))),
            0x9D => Some(Instruction::Sbc(Target::Reg8(Reg8::L))),
            0x9D => Some(Instruction::Sbc(Target::Reg16Indirect(Reg16::HL))),
            0x9F => Some(Instruction::Sbc(Target::Reg8(Reg8::A))),

            0xA0 => Some(Instruction::And(Target::Reg8(Reg8::B))),
            0xA1 => Some(Instruction::And(Target::Reg8(Reg8::C))),
            0xA2 => Some(Instruction::And(Target::Reg8(Reg8::D))),
            0xA3 => Some(Instruction::And(Target::Reg8(Reg8::E))),
            0xA4 => Some(Instruction::And(Target::Reg8(Reg8::H))),
            0xA5 => Some(Instruction::And(Target::Reg8(Reg8::L))),
            0xA6 => Some(Instruction::And(Target::Reg16Indirect(Reg16::HL))),
            0xA7 => Some(Instruction::And(Target::Reg8(Reg8::A))),

            0xA8 => Some(Instruction::Xor(Target::Reg8(Reg8::B))),
            0xA9 => Some(Instruction::Xor(Target::Reg8(Reg8::C))),
            0xAA => Some(Instruction::Xor(Target::Reg8(Reg8::D))),
            0xAB => Some(Instruction::Xor(Target::Reg8(Reg8::E))),
            0xAC => Some(Instruction::Xor(Target::Reg8(Reg8::H))),
            0xAD => Some(Instruction::Xor(Target::Reg8(Reg8::L))),
            0xAE => Some(Instruction::Xor(Target::Reg16Indirect(Reg16::HL))),
            0xAF => Some(Instruction::Xor(Target::Reg8(Reg8::A))),

            0xB0 => Some(Instruction::Or(Target::Reg8(Reg8::B))),
            0xB1 => Some(Instruction::Or(Target::Reg8(Reg8::C))),
            0xB2 => Some(Instruction::Or(Target::Reg8(Reg8::D))),
            0xB3 => Some(Instruction::Or(Target::Reg8(Reg8::E))),
            0xB4 => Some(Instruction::Or(Target::Reg8(Reg8::H))),
            0xB5 => Some(Instruction::Or(Target::Reg8(Reg8::L))),
            0xB6 => Some(Instruction::Or(Target::Reg16Indirect(Reg16::HL))),
            0xB7 => Some(Instruction::Or(Target::Reg8(Reg8::A))),

            0xB8 => Some(Instruction::Cp(Target::Reg8(Reg8::B))),
            0xB9 => Some(Instruction::Cp(Target::Reg8(Reg8::C))),
            0xBA => Some(Instruction::Cp(Target::Reg8(Reg8::D))),
            0xBB => Some(Instruction::Cp(Target::Reg8(Reg8::E))),
            0xBC => Some(Instruction::Cp(Target::Reg8(Reg8::H))),
            0xBD => Some(Instruction::Cp(Target::Reg8(Reg8::L))),
            0xBE => Some(Instruction::Cp(Target::Reg16Indirect(Reg16::HL))),
            0xBF => Some(Instruction::Cp(Target::Reg8(Reg8::A))),

            _ => { /* Add more instructions */ None }
        }
    }
}

pub struct CPU {
    pub registers: reg::Registers,
    pc: u16,
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
            pc: 0,
            bus: memory::MemoryBus {
                memory: [0; 0xFFFF]
            }
        }
    }

    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;

        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc.wrapping_add(1));
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        } else {
            panic!("Unknown instruction: 0x{:x}", instruction_byte);
        };

        self.pc = next_pc;
    }

    // executes an instruction decoded by the step() method
    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::Nop => self.nop(),
            Instruction::Cpl => self.cpl(),
            Instruction::Ccf => self.ccf(),
            Instruction::Scf => self.scf(),
            Instruction::Rlca => self.rlca(),
            Instruction::Rla => self.rla(),
            Instruction::Rrca => self.rrca(),
            Instruction::Rra => self.rra(),
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
            Instruction::Add(target) => self.perform(CPU::add, target),
            Instruction::Adc(target) => self.perform(CPU::adc, target),
            Instruction::Sub(target) => self.perform(CPU::sub, target),
            Instruction::Sbc(target) => self.perform(CPU::sbc, target),
            Instruction::Or(target) => self.perform(CPU::or, target),
            Instruction::And(target) => self.perform(CPU::and, target),
            Instruction::Xor(target) => self.perform(CPU::xor, target),
            Instruction::Cp(target) => self.perform(CPU::cp, target),
            _ => { /* Add more instructions */ self.pc }
        }

    }

    fn perform(&mut self, f: fn(&mut CPU, u8), target: Target) -> u16 {
        match target {
            Target::Reg8(r) => {
                let load = *self.reg8_lookup(r);
                f(self, load);
                self.pc.wrapping_add(1)
            }
            Target::Reg16Indirect(r) => {
                let addr = self.reg16_lookup(r);
                let load = self.bus.read_byte(addr);
                f(self, load);
                self.pc.wrapping_add(1)
            }
            _ => { self.pc }
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
        }
    }

    fn reg16_lookup(&self, register: Reg16) -> u16 {
        match register {
            Reg16::AF => self.registers.get_af(),
            Reg16::BC => self.registers.get_bc(),
            Reg16::DE => self.registers.get_de(),
            Reg16::HL => self.registers.get_hl(),
        }
    }

    fn from_target(&mut self, target: Target) -> Option<&mut u8> {
        match target {
            Target::Reg8(r) => {
                Some(self.reg8_lookup(r))
            }
            Target::Reg16Indirect(r) => {
                let addr: u16 = self.reg16_lookup(r);
                Some(self.bus.get_ref(addr))
            }
            _ => { None }
        }
    }
    
    fn add(&mut self, value: u8) {
        let (result, did_overflow) = self.registers.a.overflowing_add(value);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        
        self.registers.a = result;
    }

    fn adc(&mut self, value: u8) {
        let carry = if self.registers.f.carry { 1 } else { 0 };
        let (result, overflow1) = self.registers.a.overflowing_add(value);
        let (result,  overflow2) = result.overflowing_add(carry);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = overflow1 || overflow2;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) + carry > 0xF;

        self.registers.a = result;
    }

    fn sub(&mut self, value: u8) {
        let (result, did_overflow) = self.registers.a.overflowing_sub(value);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = ((self.registers.a & 0xF) as i8) - ((value & 0xF) as i8) < 0;

        self.registers.a = result;
    }

    fn sbc(&mut self, value: u8) {
        let carry = if self.registers.f.carry { 1 } else { 0 };
        let (result, overflow1) = self.registers.a.overflowing_sub(value);
        let (result, overflow2) = result.overflowing_sub(carry);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = overflow1 || overflow2;
        self.registers.f.half_carry = ((self.registers.a & 0xF) as i8) - ((value & 0xF) as i8) - (carry as i8) < 0;

        self.registers.a = result;
    }

    fn or(&mut self, value: u8)  {
        let result = self.registers.a | value;
        
        self.registers.f.clear_all();
        self.registers.f.zero = result == 0;

        self.registers.a = result;
    }

    fn and(&mut self, value: u8) {
        let result = self.registers.a & value;

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = true;

        self.registers.a = result;
    }

    fn xor(&mut self, value: u8) {
        let result = self.registers.a ^ value;

        self.registers.f.clear_all();
        self.registers.f.zero = result == 0;

        self.registers.a = result;
    }

    fn cp(&mut self, value: u8) {
        let (result, did_overflow) = self.registers.a.overflowing_sub(value);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = ((self.registers.a & 0xF) as i8) - ((value & 0xF) as i8) < 0;

        self.registers.a = result;
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
        let carry_bit: u8 = if self.registers.f.carry == true { 1 } else { 0 };
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
        let carry_bit: u8 = if self.registers.f.carry == true { 1 } else { 0 };
        self.registers.a >>= 1;
        self.registers.a |= (carry_bit << 7);

        self.registers.f.clear_all();
        self.registers.f.carry = bit0 != 0;

        self.pc.wrapping_add(1)
    }

    pub fn inc(&mut self, target: Target) -> u16 {
        let byte_ref: Option<&mut u8> = self.from_target(target);

        if let Some(byte) = byte_ref {
            let prior = *byte;
            *byte = byte.wrapping_add(1);

            self.registers.f.zero = *byte == 0;
            self.registers.f.subtract = false;
            self.registers.f.half_carry = (prior & 0xF) + 1 > 0xF;

            self.pc.wrapping_add(1)
        } else {
            panic!("INC unknown target");
        }
    }

    fn dec(&mut self, target: Target) -> u16 {
        let byte_ref: Option<&mut u8> = self.from_target(target);

        if let Some(byte) = byte_ref {
            let prior = *byte;
            *byte = byte.wrapping_sub(1);

            self.registers.f.zero = *byte == 0;
            self.registers.f.half_carry = ((prior & 0xF) as i8) - (1 as i8) < 0;
            self.registers.f.subtract = true;
            
            self.pc.wrapping_add(1)
        } else {
            panic!("DEC unknown target");
        }
    }

    fn rlc(&mut self, target: Target) -> u16 {
        let carry: u8 = if self.registers.f.carry == true { 1 } else { 0 };
        let byte_ref: Option<&mut u8> = self.from_target(target);

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
        let carry: u8 = if self.registers.f.carry == true { 1 } else { 0 };
        let byte_ref: Option<&mut u8> = self.from_target(target);

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
        let carry: u8 = if self.registers.f.carry == true { 1 } else { 0 };
        let byte_ref: Option<&mut u8> = self.from_target(target);

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
        let carry: u8 = if self.registers.f.carry == true { 1 } else { 0 };
        let byte_ref: Option<&mut u8> = self.from_target(target);

        if let Some(byte) = byte_ref {
            let bit0: u8 = *byte * 0x1;
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
        let byte_ref: Option<&mut u8> = self.from_target(target);

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
        let byte_ref: Option<&mut u8> = self.from_target(target);

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
        let byte_ref: Option<&mut u8> = self.from_target(target);

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
        let byte_ref: Option<&mut u8> = self.from_target(target);

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

}