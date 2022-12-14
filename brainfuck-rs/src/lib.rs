mod utils;

use std::{fmt::Display, collections::VecDeque};

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Debug, Clone)]
pub enum BrainfuckError {
    BadInput(char),
    NoProgramLoaded,
    MemoryOverflow,
    MemoryUnderflow,
    InputEOF,
    UnmatchedJump,
}

impl std::error::Error for BrainfuckError {}

impl Display for BrainfuckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadInput(c) =>
                write!(f, "BadInput: Invalid character encountered '{}'", c),
            Self::NoProgramLoaded =>
                write!(f, "NoProgramLoaded: No program loaded!"),
            Self::MemoryOverflow =>
                write!(f, "MemoryOverflow: Reached memory limit!"),
            Self::MemoryUnderflow =>
                write!(f, "MemoryUnderflow: Reached memory limit!"),
            Self::InputEOF =>
                write!(f, "InputEOF: No more input left!"),
            Self::UnmatchedJump =>
                write!(f, "UnmatchedJump: Make sure to close your brackets!"),
        }
    }
}

const MEMORY_SIZE: usize = 65536;

#[wasm_bindgen]
pub struct VirtualMachine {
    program: Option<Vec<u8>>,   // program memory
    pc: usize,                  // program counter
    memory: [u8; MEMORY_SIZE],  // 16-bit memory space
    ap: u16,                    // address pointer
    input_buffer: VecDeque<u8>  // Input buffer
}

impl VirtualMachine {
    pub fn load(&mut self, program: &str) -> Result<(), BrainfuckError> {
        self.program = Some(program.to_owned().into_bytes());
        Ok(())
    }

    pub fn input(&mut self, input: &[u8]) {
        
        self.input_buffer.append(&mut input.to_owned().into());
    }

    pub fn run(&mut self) -> Result<Vec<u8>, BrainfuckError> {
        if self.program.is_none() || self.program.as_ref().unwrap().is_empty() {
            return Err(BrainfuckError::NoProgramLoaded);
        }

        let mut output: Vec<u8> = Vec::new();
        let program = self.program.as_ref().unwrap();
        let program_len = program.len();
        loop {
            let instruction = program[self.pc];
            log(&format!("STATE\n\tpc = {} ({})\n\tap = {} [{}]", 
                self.pc, instruction as char, self.ap, self.memory[self.ap as usize]));
            match instruction {
                b'<' => {
                    self.ap = match self.ap.checked_sub(1) {
                        Some(value) => value,
                        None => return Err(BrainfuckError::MemoryUnderflow),
                    };
                },
                b'>' => {
                    self.ap = match self.ap.checked_add(1) {
                        Some(value) => value,
                        None => return Err(BrainfuckError::MemoryUnderflow),
                    };
                },
                b'+' => {
                    self.memory[self.ap as usize] += 1;
                },
                b'-' => {
                    self.memory[self.ap as usize] -= 1;
                },
                b'.' => {
                    output.push(self.memory[self.ap as usize].to_owned());
                },
                b',' => {
                    if let Some(byte) = self.input_buffer.pop_front() {
                        self.memory[self.ap as usize] = byte;
                    } else {
                        self.memory[self.ap as usize] = 0u8;
                    }
                }
                b'[' => {
                    if self.memory[self.ap as usize] == 0u8 {
                        let mut brackets = 0;
                        let mut address = self.pc;
                        loop {
                            address += 1;
                            if address == program_len {
                                return Err(BrainfuckError::UnmatchedJump);
                            }
                            match program[address] {
                                b'[' => brackets += 1,
                                b']' => {
                                    if brackets == 0 {
                                        break;
                                    }
                                    brackets -= 1;
                                }
                                _ => {}
                            }
                        }
                        self.pc = address;
                    }
                },
                b']' => {
                    if self.memory[self.ap as usize] != 0u8 {
                        let mut brackets = 0;
                        let mut address = self.pc;
                        loop {
                            if address == 0 {
                                return Err(BrainfuckError::UnmatchedJump);
                            }
                            address -= 1;
                            match program[address] {
                                b'[' => {
                                    if brackets == 0 {
                                        break;
                                    }
                                    brackets -= 1;
                                },
                                b']' => brackets += 1,
                                _ => {}
                            }
                        }
                        self.pc = address;
                    }
                },
                _ => panic!("Invalid instruction encountered!"),
            }
            self.pc += 1;
            if self.pc == program_len {
                break;
            }
        }
        Ok(output)
    }
}

#[wasm_bindgen]
impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            program: None,
            pc: 0usize,
            memory: [0u8; 65536],
            ap: 0u16,
            input_buffer: VecDeque::new(),
        }
    }

    pub fn set_address_pointer_js(&mut self, address: u16) {
        self.ap = address;
    }

    pub fn load_js(&mut self, program: &str) -> Result<(), JsError> {
        let allowed_chars = "<>+-.,[]";
        let filtered_string = program.chars().filter(|c| {
            allowed_chars.contains(*c)
        }).collect::<String>();
        log(&filtered_string);
        self.load(filtered_string.as_str())?;
        Ok(())
    }

    pub fn input_js(&mut self, input: &str) {
        self.input(input.as_bytes());
    }

    pub fn run_js(&mut self) -> Result<String, JsError> {
        let output = self.run()?;

        if let Ok(output_string) = String::from_utf8(output) {
            Ok(output_string)
        } else {
            Err(JsError::new("InvalidAscii"))
        }
    }
}
