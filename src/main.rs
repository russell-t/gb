#![allow(unused)] // temporarily allow unused variables, functions, methods

mod reg;
mod cpu;
mod memory;

use std::fs;

fn main() {
    match std::fs::read("./09-op r,r.gb") {
        Ok(bytes) => {
            for (i, &item) in bytes.iter().enumerate() {
                println!("{:x} {:x}", i, item);
            }
        }
        Err(e ) => {
            panic!("{}", e);
        }
    }
    let mut gb_cpu = cpu::CPU::new();
}
