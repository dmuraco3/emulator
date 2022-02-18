use super::memory::Memory;
use super::register::Registers;
use std::fs::{metadata ,File};
use std::io::{BufWriter, Cursor, Write, Result};
use byteorder::{ReadBytesExt, BigEndian, NativeEndian, LittleEndian};


const START: u16 = 0x3000;

const MEMORY_SIZE: usize = u16::MAX as usize;

pub struct VM {
    pub memory    : Memory,
    pub registers : Registers
}

impl VM {
    pub fn new() -> VM {
        VM {
            memory:    Memory::new(),
            registers: Registers::new(),
        }
    }
    pub fn sign_extend(&self, mut x: u16, bit_count: i16) -> u16{
      if ((x >> (bit_count - 1)) & 1) == 1{
          x |= (0xFFFF << bit_count);
      }
      return x;
    }
    pub fn update_flags(&mut self, r_num: u16) {
      let t = self.registers.read(r_num);
      if t == 0 {
        self.registers.update(9, 1 << 1);
      } else if t >> 15 == 1 {
        self.registers.update(9, 1 << 2);
      } else {
        self.registers.update(9, 1 << 0);
      }
    }
  
    pub fn add(&mut self, instruction: u16) {
      let dr = (instruction  >> 9) & 0x7;
      let r0 = (instruction >> 6) & 0x7;

      let imm_flag = (instruction >> 5) & 0x1;
      if imm_flag != 0 {
        let imm5 = self.sign_extend(instruction & 0x1F, 5);
        self.registers.update(dr, self.registers.read(r0) + imm5);
          
      } else {
        let r1 = instruction & 0x7;
        self.registers.update(dr, self.registers.read(r0) + self.registers.read(r1))
      }
      let d = self.registers.read(dr);
    }
    pub fn read(&mut self, path: String) {
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
    pub fn execute(&mut self) {
        let mut start = 0x3000;
        loop {
            match self.memory.read(start) {
              0 => break,
              (instruction) => {
                let opcode = instruction >> 12;
                  
                match opcode {
                  0 => {},
                  1 => {self.add(instruction)},
                  2 => {},
                  3 => {},
                  4 => {},
                  5 => {},
                  6 => {},
                  7 => {},
                  8 => {},
                  9 => {},
                  10 => {},
                  11 => {},
                  12 => {},
                  13 => {},
                  14 => {},
                  15 => {},
                  _ => panic!("BAD OPCODE : {} at {:x}", opcode, start)
                }
                start += 1;
              } 
            }
            let instruction = self.memory.read(start);
            let opcode = instruction >> 12;
        }
    }
}