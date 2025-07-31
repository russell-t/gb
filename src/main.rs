#![allow(unused)] // temporarily allow unused variables, functions, methods

mod reg;
mod cpu;
mod memory;

fn main() {
    let fr = reg::FlagsRegister {
        zero: true,
        subtract: false,
        half_carry: false,
        carry: true
    };
    let foo: u8 = fr.into();
    println!("{:b}", foo);
}
