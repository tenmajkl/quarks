use std::path::Path;
use std::fs;
// Absolute survielence over programming languages
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

fn parse(program: Vec<Token>)
{
    let mut mode = Mode::Native;
    let mut stack = Vec::new();
    let mut stack_size = 0;
    for token in program
    {
        match mode {
            Mode::Native => {
                match token {
                    Token::Single => mode = Mode::Stack,
                    Token::Double => mode = Mode::Math,
                    Token::Rev => mode = Mode::Misc,
                }
            },
            Mode::Stack => {
                match token {
                    Token::Single => mode = Mode::Native,
                    Token::Double => {
                        stack_size += 1;
                        stack.push(0)
                    },
                    Token::Rev => {
                        stack_size -= 1;
                        drop(stack.pop())
                    }
                }
            },
            Mode::Math => {
                match token {
                    Token::Double => mode = Mode::Native,
                    Token::Single => stack[stack_size - 1] += 1,
                    Token::Rev => stack[stack_size - 1] -= 1
                }
            },
            Mode::Misc => {
                match token {
                    Token::Rev => mode = Mode::Native,
                    Token::Single => println!("{}", stack[stack_size -1]),
                    Token::Double => println!("Loops are not implemented!") 
                }
            }
        }
    }
}

fn main() 
{
    parse(lex("'\"'`'`")) 
}

/* 
mody:
    Nativni
    - enter - enter - ukonci
            - mezera - push
            - nl - pop

    - mezera - mezera - konec
             - enter - +
             - nl - -
    - tan - tab - konec
         - mezera - zapne/skonci cyklus
         - enter - tisk ve stacku
*/
