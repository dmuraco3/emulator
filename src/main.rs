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

pub mod hardware;
use hardware::vm::VM;

/*
  0001 000 001 0 00 010

  TODO: 
    to write to file we need to encode rust to ascii 
    python does this with bytes.fromhex("<HEX NUMBER HERE>")
        LOOK AT ../conv.py
    so yeah
*/


fn main() {

    let mut vm = VM::new();
    let path = "./file.obj".to_owned();
    vm.read(path);
    vm.registers.update(1, 5);
    vm.registers.update(2, 3);
    vm.execute();
    println!("TEST: {}", vm.registers.read(2))
}
