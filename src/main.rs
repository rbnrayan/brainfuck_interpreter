use std::{fs::File, env, io::Read};

const MEM_LENGTH: usize = 65535;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run <file>");
    }

    let mut f = File::open(&args[1]).expect("Failed to open the file");
    let mut buf = String::new();

    f.read_to_string(&mut buf).expect("Failed to read the file to string");
    Brainfuck::new(buf)
        .interpret();
}

struct Brainfuck {
    raw_code: String,
    memory: [u8; MEM_LENGTH],
    mem_ptr: usize,
}

impl Brainfuck {
    fn new(src: String) -> Self {
        Brainfuck {
            raw_code: src,
            memory: [0; MEM_LENGTH],
            mem_ptr: 0,
        }
    }

    fn interpret(&mut self) {
        let mut src_ptr = 0;
        let src = self.raw_code.as_bytes();
        let mut jump_table = Vec::new();

        while src_ptr < src.len() {
            match src[src_ptr] {
                b'>' => {
                    if self.mem_ptr == MEM_LENGTH - 1 { self.mem_ptr = 0; }
                    else { self.mem_ptr += 1; }
                },
                b'<' => {
                    if self.mem_ptr == 0 { self.mem_ptr = MEM_LENGTH - 1; }
                    else { self.mem_ptr -= 1; }
                },
                b'+' => {
                    if self.memory[self.mem_ptr] == 255 {
                        self.memory[self.mem_ptr] = 0;
                    } else {
                        self.memory[self.mem_ptr] += 1;
                    }
                },
                b'-' => {
                    if self.memory[self.mem_ptr] == 0 {
                        self.memory[self.mem_ptr] = 255;
                    } else {
                        self.memory[self.mem_ptr] -= 1;
                    }
                },
                b'.' => {
                    print!("{}", self.memory[self.mem_ptr] as char);
                },
                b'[' => {
                    if self.memory[self.mem_ptr] == 0 {
                        while src[src_ptr] != b']' {
                            src_ptr += 1;
                        }
                    } else {
                        jump_table.push(src_ptr);
                    }
                },
                b']' => {
                    if self.memory[self.mem_ptr] != 0 {
                        src_ptr = *jump_table.iter().last().expect("Failed to get the last value of the jump table. (Hint: add a closing bracket ']')");
                    } else {
                        jump_table.pop();
                    }
                },
                b'\n' | b'\r' => {},
                unexpected_char => panic!("Found an unexpected character: {}", unexpected_char),
            }
            src_ptr += 1;
        }
    }
}
