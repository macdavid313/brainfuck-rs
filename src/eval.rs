#[derive(Debug)]
pub enum Instruction {
    MoveRight,
    MoveLeft,
    Inc,
    Dec,
    Putchar,
    Getchar,
    WhileNotZero(Vec<Instruction>),
}

#[derive(Debug)]
pub struct BrainfuckVM {
    memory_block_limit: i32,
    operation_limit: i32,
    operation_n: i32,
    pointer: usize,
    memory: Vec<u8>,
}

impl BrainfuckVM {
    fn new(memory_block_limit: i32, operation_limit: i32) -> BrainfuckVM {
        BrainfuckVM {
            memory_block_limit,
            operation_limit,
            operation_n: 0,
            pointer: 0,
            memory: vec![0; memory_block_limit as usize],
        }
    }
    
    fn get_data(&self) -> u8 {
        self.memory[self.pointer]
    }

    fn move_right(&mut self) {
        self.pointer += 1;
        self.operation_n += 1;
    }

    fn move_left(&mut self) {
        self.pointer -= 1;
        self.operation_n += 1;
    }

    fn inc(&mut self) {
        if self.memory[self.pointer] == 255 {
            self.memory[self.pointer] = 0;
        } else {
            self.memory[self.pointer] += 1;
        }
        self.operation_n += 1;
    }

    fn dec(&mut self) {
        if self.memory[self.pointer] == 0 {
            self.memory[self.pointer] = 255;
        } else {
            self.memory[self.pointer] -= 1;
        }
        self.operation_n += 1;
    }

    fn putchar(&mut self) {
        print!("{}", self.memory[self.pointer] as char);
        self.operation_n += 1;
    }

    fn getchar(&self) {
        unimplemented!();
        self.operation_n += 1;
    }

    fn operation_limit_exceeded(&self) -> bool {
        self.operation_n > self.operation_limit
    }
}

pub fn parse(program: &str) -> Vec<Instruction> {
    fn _parse(program: &str) -> (Vec<Instruction>, usize) {
        let mut res: Vec<Instruction> = Vec::new();
        let chars: Vec<char> = program.chars().collect();
        let (mut i, len) = (0usize, program.len());
        while i < len {
            match chars[i] {
                '>' => res.push(Instruction::MoveRight),
                '<' => res.push(Instruction::MoveLeft),
                '+' => res.push(Instruction::Inc),
                '-' => res.push(Instruction::Dec),
                '.' => res.push(Instruction::Putchar),
                ',' => res.push(Instruction::Getchar),
                '[' => {
                    let (block, offset) = _parse(&program[i + 1..]);
                    i += offset;
                    res.push(Instruction::WhileNotZero(block));
                }
                ']' => return (res, i + 1),
                _ => (),
            }
            i += 1;
        }
        (res, 0)
    }
    let (res, _) = _parse(program);
    res
}

fn eval_by_vm(instructions: &[Instruction], vm: &mut BrainfuckVM) {
    for instruction in instructions {
        match instruction {
            Instruction::MoveRight => vm.move_right(),
            Instruction::MoveLeft => vm.move_left(),
            Instruction::Inc => vm.inc(),
            Instruction::Dec => vm.dec(),
            Instruction::Putchar => vm.putchar(),
            Instruction::Getchar => vm.getchar(),
            Instruction::WhileNotZero(block) => {
                while vm.get_data() != 0 {
                    eval_by_vm(block, vm);
                }
            },
            _ => continue
        }
        if vm.operation_limit_exceeded() {
            println!( "PROCESS TIME OUT. KILLED!!!" );
            break
        }
    }
}

pub fn eval(program: String) {
    let mut vm = BrainfuckVM::new(100_000, 100_000);
    eval_by_vm(&parse(&program), &mut vm);
}

#[cfg(test)]
mod tests {
    use super::parse;
    #[test]
    fn test_parse() {}
}
