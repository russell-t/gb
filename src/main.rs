#![allow(unused)] // temporarily allow unused variables, functions, methods

mod reg;
mod cpu;
mod memory;

use std::fs;

fn main() {

    /* Create an instance of a CPU */
    let mut gb_cpu = cpu::CPU::new();

    /* Instructions Under Test */
    let mut iut: Vec<u8> = (0x80..0xC0).collect();
    let iut_additional: [u8; 23] = [0x04, 0x05, 0x07, 0x14, 0x15, 0x17, 0x24, 0x25, 0x34, 0x35, 0x37,
                                   0x0C, 0x0D, 0x0F, 0x1C, 0x1F, 0x1D, 0x2C, 0x2D, 0x2F, 0x3C, 0x3D,
                                   0x3F];
    iut.extend(&iut_additional);

    let iut_prefixed: Vec<u8> = (0x00..0x40).collect();

    for i in iut {
        let tests: Vec<cpu::CpuTest> = serde_json::from_str::<Vec<cpu::CpuTest>>(
                &String::from_utf8(
                    std::fs::read(
                        format!("sm83/v1/{:02x}.json", i)
                    ).unwrap()
                ).unwrap()
            ).unwrap();
        for test in tests {
            gb_cpu.set_state(&test.initial_state);
            let instruction: cpu::Instruction = cpu::Instruction::from_byte(i, false).unwrap();
            let pc = gb_cpu.execute(instruction);
            gb_cpu.pc = pc;
            gb_cpu.compare_state(&test.final_state);
        }
        println!("0x{:02x} passed!", i);
    }

    for i in iut_prefixed {
        let tests: Vec<cpu::CpuTest> = serde_json::from_str::<Vec<cpu::CpuTest>>(
                &String::from_utf8(
                    std::fs::read(
                        format!("sm83/v1/cb {:02x}.json", i)
                    ).unwrap()
                ).unwrap()
            ).unwrap();
        for test in tests {
            gb_cpu.set_state(&test.initial_state);
            let instruction: cpu::Instruction = cpu::Instruction::from_byte(i, true).unwrap();
            let pc = gb_cpu.execute(instruction);
            gb_cpu.pc = pc;
            gb_cpu.compare_state(&test.final_state);
        }
        println!("0xcb{:02x} passed!", i);
    }

}
