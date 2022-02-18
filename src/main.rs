/*
registers:

    r0-r7 - General Registers
    pc    - Program Counter
    cond  - Condition Flag Register
*/

/*
instructions:
    ADD r2, r1, #5
*/

use std::fs::{metadata ,File};
use std::io::{BufWriter, Cursor, Write, Result};
use byteorder::{ReadBytesExt, BigEndian, NativeEndian, LittleEndian};

const START: u16 = 0x3000;

const MEMORY_SIZE: usize = u16::MAX as usize;


struct Registers {
    r0:   u16,
    r1:   u16,
    r2:   u16,
    r3:   u16,
    r4:   u16,
    r5:   u16,
    r6:   u16,
    r7:   u16,
    pc:   u16,
    cond: u16,
}

impl Registers {
    fn new() -> Registers {
        Registers {
            r0:   0,
            r1:   0,
            r2:   0,
            r3:   0,
            r4:   0,
            r5:   0,
            r6:   0,
            r7:   0,
            pc:   START,
            cond: 0,
        }
    }

    fn read(&self, index: u16) -> u16 {
        match index {
            0 => {self.r0},
            1 => {self.r1},
            2 => {self.r2},
            3 => {self.r3},
            4 => {self.r4},
            5 => {self.r5},
            6 => {self.r6},
            7 => {self.r7},
            8 => {self.pc},
            9 => {self.cond}
            _ => {panic!("Index out of range for register {}", index)}
        }
    }

    fn update(&mut self, index: u16, value: u16) {
        match index {
            0 => {self.r0   = value},
            1 => {self.r1   = value},
            2 => {self.r2   = value},
            3 => {self.r3   = value},
            4 => {self.r4   = value},
            5 => {self.r5   = value},
            6 => {self.r6   = value},
            7 => {self.r7   = value},
            8 => {self.pc   = value},
            9 => {self.cond = value}
            _ => {panic!("Index out of range for register {}", index)}
        }
    }
}

struct Memory {
    memory: [u16; MEMORY_SIZE]
}
impl Memory {
    fn new () -> Memory {
        Memory {
            memory: [0;MEMORY_SIZE]
        }
    }
    
    fn read(&self, address: u16) -> u16 {
        return self.memory[address as usize];
    }

    fn update(&mut self, address: u16, val: u16) {
        self.memory[address as usize] = val;
    }
}

struct VM {
    memory    : Memory,
    registers : Registers
}

impl VM {
    fn new() -> VM {
        VM {
            memory:    Memory::new(),
            registers: Registers::new(),
        }
    }
    fn read(&mut self, path: String) {
        let mut f = File::open(path).expect("no file found");
        let base_address = f.read_u16::<BigEndian>().expect("ERROR");
        let mut address = base_address;
        loop {
            match f.read_u16::<BigEndian>() {
                Ok(instruction) => {
                    // println!("mem : {}", address);
                    self.memory.update(address, instruction);
                    address += 1;
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    } else {
                        panic!("ERROR: {}", e)
                    }
                    break
                }
            }
        }
    }
    fn execute(&self) {
        let start = self.memory.read(0x3000);
        let opcode = start >> 12;
        match opcode {
            1 => {}
            _ => {panic!("BAD OPCODE")}
        }
    }

}

/*
  TODO: 
    to write to file we need to encode rust to ascii 
    python does this with bytes.fromhex("<HEX NUMBER HERE>")
        LOOK AT ../conv.py
    so yeah
*/


fn main() {

    let mut vm = VM::new();
    let path = "./txt.obj".to_owned();
    vm.read(path);
    vm.execute();
}
