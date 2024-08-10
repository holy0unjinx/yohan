mod lexer;

use lexer::{Lexer, Token};
use std::io::{self, Write};

fn main() {
  loop {
    let mut input = String::new();
    print!("> ");
    io::stdout().flush().expect("출력을 플러시하는데 실패했습니다.");

    io::stdin()
      .read_line(&mut input)
      .expect("입력을 읽는데 실패했습니다.");

    let input = input.trim();

    if input == "exit" {
      break;
    }

    let mut lexer = Lexer::new(input);

    loop {
      let token = lexer.get_next_token();
      println!("{:?}", token);
      if token == Token::EOF {
        break;
      }
    }
  }
}