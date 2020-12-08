use util::*;

mod computer;
use computer::Instruction;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer = Timer::new();
    let original = input::into::<computer::Computer>(&std::env::args().nth(1).unwrap());
    for (i, inst) in original.memory().iter().enumerate() {
        let new_inst;
        match inst {
            Instruction::Nop(n) => new_inst = Instruction::Jmp(*n),
            Instruction::Jmp(n) => new_inst = Instruction::Nop(*n),
            _ => continue,
        }
        let mut computer = original.clone();
        let memory = computer.memory_mut();
        memory[i] = new_inst;
        computer.run_until_loop();
        if computer.pc() == computer.len() {
            timer.print();
            println!("{}", computer.acc());
            break;
        }
    }
    Ok(())
}
