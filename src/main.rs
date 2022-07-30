use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use std::fs::File; 
use std::env; 
use std::io::{Read, self};

const MEM_LENGTH: usize = 65535;

fn main() {
    let stdin = 0;
    let termios = Termios::from_fd(stdin)
        .expect("Failed to create a new termios from the file descriptor");
    let mut termios_c = termios.clone();
    termios_c.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &mut termios_c)
        .expect("Failed to change termios attributes immediately");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run <file>");
    }

    let mut f = File::open(&args[1]).expect("Failed to open the file");
    let mut buf = String::new();

    f.read_to_string(&mut buf).expect("Failed to read the file to string");
    Brainfuck::new(buf)
        .interpret();

    tcsetattr(stdin, TCSANOW, &termios)
        .expect("Failed to reset termios attributes to default");
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
                b',' => {
                    let mut buf: [u8; 1] = [0];
                    io::stdin().read_exact(&mut buf).expect("Failed to read a character");
                    self.memory[self.mem_ptr] = buf[0];
                }
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
                        src_ptr = *jump_table.iter().last().expect("Failed to get the last value of the jump table.");
                    } else {
                        jump_table.pop();
                    }
                },
                _ => {},
            }
            src_ptr += 1;
        }
    }
}
