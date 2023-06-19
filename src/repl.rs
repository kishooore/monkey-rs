use std::io::Write;

use crate::lexer::Lexer;


const PROMPT: &str = ">> ";

pub fn start() {
    let username = whoami::username();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("Hello {username}! This is monkey programming language!");
    println!("feel free to type in some commands!");
    print!("{PROMPT}");
    std::io::stdout().flush().unwrap();
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();

        if buf.is_empty() {
            continue;
        }

        let mut lexer = Lexer::new(buf);

        let mut token = lexer.next_token();

        while token != None {
            println!("{:?}", token.unwrap());
            token = lexer.next_token();
        }

        print!("{PROMPT}");
        std::io::stdout().flush().unwrap();
    }
}
