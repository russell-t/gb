use crate::reg;
use crate::memory;

enum Target {
    Reg8(Reg8),
    Reg16(Reg16),
    Reg16Indirect(Reg16),
}

enum Reg8 {
    A, B, C, D, E, H, L,
}

// af, bc, de, hl
enum Reg16 {
    AF, BC, DE, HL, 
}

enum Instruction {
    Add(Target),
    Adc(Target),
    Sub(Target),
    Sbc(Target),
    Or(Target),
    And(Target),
    Xor(Target),
    Cp(Target),
    Cpl,
    Ccf,
    Scf,
    Nop,
    Rlca,
}

impl Instruction {
    fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::Nop),
            0x07 => Some(Instruction::Rlca),
            0x2F => Some(Instruction::Cpl),
            0x37 => Some(Instruction::Scf),
            0x3F => Some(Instruction::Cpl),

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

struct CPU {
    registers: reg::Registers,
    pc: u16,
    bus: memory::MemoryBus,
}

impl CPU {

    fn step(&mut self) {
        let instruction_byte = self.bus.read_byte(self.pc);

        if let Some(instruction) = Instruction::from_byte(instruction_byte) {
            self.pc = self.execute(instruction);
        } else {
            panic!("Unknown instruction: 0x{:x}", instruction_byte);
        };
    }

    // executes an instruction decoded by the step() method
    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::Nop => self.nop(),
            Instruction::Cpl => self.cpl(),
            Instruction::Ccf => self.ccf(),
            Instruction::Scf => self.scf(),
            Instruction::Rlca => self.rlca(),
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
                let load = self.reg8_lookup(r);
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

    fn reg8_lookup(&self, register: Reg8) -> u8 {
        match register {
            Reg8::B => self.registers.b,
            Reg8::C => self.registers.c,
            Reg8::D => self.registers.d,
            Reg8::E => self.registers.e,
            Reg8::H => self.registers.h,
            Reg8::L => self.registers.l,
            Reg8::A => self.registers.a,
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
        
        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;

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

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;

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
        let bit7 = if (self.registers.a & 0x80) > 0 {1} else { 0 };
        self.registers.a = self.registers.a << 1;
        self.registers.a |= bit7;

        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = bit7 != 0;

        self.pc.wrapping_add(1)
    }
}