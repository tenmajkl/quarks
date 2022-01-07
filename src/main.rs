// Absolute survielence over programming languages
use std::path::Path;
use std::fs;
use std::env;

enum Mode 
{
    Native,
    Stack,
    Math,
    Misc
}
// TODO Better names
enum Token
{
    Single,
    Double,
    Rev
}

/**
 * Executes lexing procedure which converts all valid characters to tokens
 */
fn lex(program: &str) -> Vec<Token>
{
    let mut tokens = Vec::new();
    for character in program.chars()
    {
        match character {
            '\'' => tokens.push(Token::Single),
            '"' => tokens.push(Token::Double),
            '`' => tokens.push(Token::Rev),
            _ => {}
        }
    }
    tokens
}

struct Simulator
{
    mode: Mode,
    stack: Vec<i32>,
    stack_size: usize,
}

impl Simulator
{
    pub fn new() -> Simulator
    {
        Simulator {
            mode: Mode::Native,
            stack: Vec::new(),
            stack_size: 0,
        }
    }
   
    /**
     * Parses all given tokens and runs whole procedure 
     */
    fn parse(&mut self, program: &Vec<Token>)
    {
        let mut parsing_loop = false;
        let mut loop_tokens:Vec<Token> = Vec::new();
        for token in program
        {

            if parsing_loop
            {
                // its stupid i know but i didnt found better way to get the value from reference
                loop_tokens.push(match token {
                    &Token::Single => Token::Single,
                    &Token::Double => Token::Double,
                    &Token::Rev => Token::Rev,
                });
            }

            match self.mode {
                Mode::Native => {
                    self.mode = match token {
                        Token::Single => Mode::Stack,
                        Token::Double =>Mode::Math,
                        Token::Rev => Mode::Misc,
                    }
                },
                Mode::Stack => {
                    match token {
                        Token::Single => self.mode = Mode::Native,
                        Token::Double => {
                            if parsing_loop { continue; }
                            self.stack_size += 1;
                            self.stack.push(0)
                        },
                        Token::Rev => {
                            if parsing_loop { continue; }
                            self.stack_size -= 1;
                            drop(self.stack.pop())
                        }
                    }
                },
                Mode::Math => {
                    match token {
                        Token::Double => self.mode = Mode::Native,
                        Token::Single => {
                            if parsing_loop { continue; }
                            self.stack[self.stack_size - 1] += 1
                        },
                        Token::Rev => { 
                            if parsing_loop { continue; }
                            self.stack[self.stack_size - 1] -= 1 
                        }
                    }
                },
                Mode::Misc => {
                    match token {
                        Token::Rev => self.mode = Mode::Native,
                        Token::Single => { 
                            if parsing_loop { continue; }
                            println!("{}", self.stack[self.stack_size -1])
                        },
                        Token::Double => {

                            if parsing_loop
                            {
                                while self.stack[self.stack_size - 1 ] != 0
                                {
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

    pub fn simulate(&mut self, program: Vec<Token>)
    {
        self.parse(&program)
    }

}

fn main() 
{
    let mut args = env::args();
    let filename = args.nth(1).unwrap();
    if !Path::new(&filename).exists()
    {
        println!("File {} not found!", filename);
        return;
    }
    let code = fs::read_to_string(filename)
        .expect("Error corrupted while reading file");

    let mut simulator = Simulator::new();
    simulator.simulate(lex(&code));

}

