// Absolute survielence over programming languages
use std::path::Path;
use std::fs;
use std::env;
use std::process::exit;

enum Mode {
    Native,
    Stack,
    Math,
    Misc
}

#[derive(Copy, Clone)]
enum TokenType {
    Single,
    Double,
    Rev
}

#[derive(Copy, Clone)]
struct Token {
    kind: TokenType,
    line: usize,
    pos: usize
}

/**
 * Executes lexing procedure which converts all valid characters to tokens
 */
fn lex(program: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut line = 1;
    let mut pos = 1;
    for character in program.chars() {
        pos += 1;
        match character {
            '\'' => tokens.push(Token { kind: TokenType::Single, line, pos }),
            '"' => tokens.push(Token { kind: TokenType::Double, line, pos }),
            '`' => tokens.push(Token { kind: TokenType::Rev, line, pos }),
            '\n' => {
                pos += 1;
                line += 1
            },
            _ => {}
        }
    }
    tokens
}

fn error(message: &str, line: usize, pos: usize) {
    println!("\x1b[31mERROR: \"{}\" at line {}, position {} \x1b[0m", message, line, pos);
    exit(1);
}

struct Simulator {
    mode: Mode,
    stack: Vec<i32>,
    stack_size: usize,
}

impl Simulator {
    pub fn new() -> Simulator {
        Simulator {
            mode: Mode::Native,
            stack: Vec::with_capacity(256),
            stack_size: 0,
        }
    }
   
    /**
     * Parses all given tokens and runs whole procedure 
     */
    fn parse(&mut self, program: &Vec<Token>) {
        let mut parsing_loop = false;
        let mut loop_tokens:Vec<Token> = Vec::new();
        for token in program {

            if parsing_loop {
                loop_tokens.push(*token)
            }

            match self.mode {
                Mode::Native => {
                    self.mode = match token.kind {
                        TokenType::Single => Mode::Stack,
                        TokenType::Double =>Mode::Math,
                        TokenType::Rev => Mode::Misc,
                    }
                },
                Mode::Stack => {
                    match token.kind {
                        TokenType::Single => self.mode = Mode::Native,
                        TokenType::Double => {
                            if parsing_loop { continue; }
                            if self.stack_size == 256
                            {
                                error("Can't push to stack, because stack is full.", token.line, token.pos)
                            }
                            self.stack_size += 1;
                            self.stack.push(0)
                        },
                        TokenType::Rev => {
                            if parsing_loop { continue; }
                            if self.stack_size == 0 {
                                error("Can't pop from stack, because stack is empty!", token.line, token.pos)
                            }
                            self.stack_size -= 1;
                            drop(self.stack.pop())
                        }
                    }
                },
                Mode::Math => {
                    match token.kind {
                        TokenType::Double => self.mode = Mode::Native,
                        TokenType::Single => {
                            if parsing_loop { continue; }
                            if self.stack_size == 0 {          
                                error("Number adding can't be done, because stack is empty", token.line, token.pos);
                            }
                            self.stack[self.stack_size - 1] += 1
                        },
                        TokenType::Rev => { 
                            if parsing_loop { continue; }
                            if self.stack_size == 0 {          
                                error("Number subtraction can't be done, because stack is empty", token.line, token.pos);
                            }
                            self.stack[self.stack_size - 1] -= 1 
                        }
                    }
                },
                Mode::Misc => {
                    match token.kind {
                        TokenType::Rev => self.mode = Mode::Native,
                        TokenType::Single => { 
                            if parsing_loop { continue; }
                            if self.stack_size == 0 {          
                                error("Printing can't be done, because stack is empty", token.line, token.pos);
                            }
                            println!("{}", self.stack[self.stack_size -1])
                        },
                        TokenType::Double => {

                            if parsing_loop {
                                while self.stack[self.stack_size - 1 ] != 0 {
                                    self.parse(&loop_tokens);
                                }
                                continue;
                            }

                            parsing_loop = true;
                        }
                    }
                }
            }
        }
    }

    pub fn simulate(&mut self, program: Vec<Token>) {
        self.parse(&program)
    }

}

fn main() {
    let mut args = env::args();
    let filename = args.nth(1).unwrap();
    if !Path::new(&filename).exists() {
        println!("File {} not found!", filename);
        return;
    }
    let code = fs::read_to_string(filename)
        .expect("Error corrupted while reading file");

    let mut simulator = Simulator::new();
    simulator.simulate(lex(&code));

}

