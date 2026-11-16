use std::fmt;

#[allow(non_snake_case)]
#[allow(dead_code)] // remove me...
struct StatusFlags {
    N: bool,
    V: bool,

    B: bool,
    D: bool,
    I: bool,
    Z: bool,
    C: bool,
}

#[allow(non_snake_case)]
struct RegisterState {
    A:  u8,
    X:  u8,
    Y:  u8,
    PC: u16,
    S:  u8,
    P:  StatusFlags,
}

impl fmt::Display for RegisterState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "A: 0x{:X}  X: 0x{:X}  Y: 0x{:X}  PC: 0x{:X}  S: 0x{:X}",
            self.A, self.X, self.Y, self.PC, self.S
        )
    }
}

fn main() {
    // Almost certainly not the correct initialization.
    let mut memory: [u8; 65536] = [0; 65536];
    let mut registers = RegisterState {
        A: 0, X: 0, Y: 0, PC: 0, S: 0xFD,
        P: StatusFlags {
            N: false, V: false, B: false,
            D: false, I: false, Z: false, C: false,
        },
    };

    // "Load" an example program.
    memory[0] = 0xE8; // INX
    memory[1] = 0xD0; // BNE
    memory[2] = 0xFF; //  -1

    loop {
        // println!("Register state:  {registers}");

        let current_instruction = memory[usize::from(registers.PC)];
        match current_instruction {
            0xE8 /* INX */ => {
                // println!("INX");

                // Everything about this is probably wrong.
                registers.X   = registers.X.wrapping_add(1);
                registers.P.N = (registers.X & 0x80) == 0x80;
                registers.P.Z = registers.X == 0;

                registers.PC += 1;
            },

            0xD0 /* BNE */ => {
                // println!("BNE");

                if registers.P.Z == false {
                    let immediate = memory[usize::from(registers.PC + 1)];
                    if immediate & 0xF0 == 0xF0 {
                        let offset = !(immediate - 1);
                        registers.PC -= u16::from(offset);
                    } else {
                        registers.PC += u16::from(immediate);
                    }
                } else {
                    registers.PC += 2;
                }
            },

            _ => {
                println!( "Unimplemented instruction" );
                break;
            },
        }
    }
}
