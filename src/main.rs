// TODO: write TODOS to know what to do
// Everything is TODO
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
    for token in program
    {
        match token {
            Token::Space => {
                match mode {
                    Mode::Native => println!("Not implemented"),
                    Mode::Stack => stack.push(0), 
                    Mode::Math => println!("Not implemented"),
                    Mode::Misc => println!("Not implemented")
                }
            },
            Token::Enter => {
                match mode {
                    Mode::Native => mode = Mode::Stack, 
                    Mode::Stack => mode = Mode::Native, 
                    Mode::Math => println!("Not implemented"),
                    Mode::Misc => println!("Not implemented")
                }
            },
            Token::Tab => {
                  match mode {
                    Mode::Native => println!("Not implemented"), 
                    Mode::Stack => drop(stack.pop()), 
                    Mode::Math => println!("Not implemented"),
                    Mode::Misc => println!("Not implemented")
                }     
            } 
        }
    }
}

fn main() 
{
    let program = [
        Token::Enter,
        Token::Tab,
        Token::Space,
        Token::Enter,
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
