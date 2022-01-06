// Absolute survielence over programming languages
enum Mode 
{
    Native,
    Stack,
    Math,
    Misc
}

enum Token
{
    Space,
    Enter,
    Tab
}

fn parse(program: &[Token])
{
    let mut mode = Mode::Native;
    let mut stack = Vec::new();
    let mut stack_size = 0;
    for token in program
    {
        match mode {
            Mode::Native => {
                match token {
                    Token::Enter => mode = Mode::Stack,
                    Token::Space => mode = Mode::Math,
                    Token::Tab => mode = Mode::Misc,
                }
            },
            Mode::Stack => {
                match token {
                    Token::Enter => mode = Mode::Native,
                    Token::Space => {
                        stack_size += 1;
                        stack.push(0)
                    },
                    Token::Tab => {
                        stack_size -= 1;
                        drop(stack.pop())
                    }
                }
            },
            Mode::Math => {
                match token {
                    Token::Space => mode = Mode::Native,
                    Token::Enter => stack[stack_size - 1] += 1,
                    Token::Tab => stack[stack_size - 1] -= 1
                }
            },
            Mode::Misc => {
                match token {
                    Token::Tab => mode = Mode::Native,
                    Token::Enter => println!("{}", stack[stack_size -1]),
                    Token::Space => println!("Loops are not implemented!") 
                }
            }
        }
    }
}

fn main() 
{
    let program = [
        Token::Enter,
            Token::Space,
        Token::Enter,
        
        Token::Space,
            Token::Enter,
            Token::Enter,
        Token::Space,
        
        Token::Tab,
            Token::Enter,
        Token::Tab
    ]; 
    parse(&program);
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
