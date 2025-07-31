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
    And(Target),
    Xor(Target),
    Or(Target),
    Cp(Target),
    Inc(Target),
    Dec(Target),
    Daa(Target),
    Scf(Target),
    Cpl(Target),
    Ccf(Target),
}

impl Instruction {
    fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
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
            Instruction::Add(target) => {
                match target {
                    Target::Reg8(r) => {
                        let value = self.reg8_lookup(r);
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    Target::Reg16Indirect(r) => {
                        let addr = self.reg16_lookup(r);
                        let value = self.bus.read_byte(addr);
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    _ => { self.pc }
                }
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
    
    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
}