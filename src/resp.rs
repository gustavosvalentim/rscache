use crate::database::{MemoryDatabase, Value};

#[derive(Debug)]
pub struct Tokenizer {
    input: String,
    position: i32,
}

impl Tokenizer {
    pub fn new(input: String) -> Tokenizer {
        Tokenizer { input, position: 0 }
    }

    pub fn next(&mut self) -> Option<char> {
        if self.peek().is_none() {
            return None;
        }

        let c = self.input.chars().nth(self.position as usize);
        self.position += 1;
        c
    }

    pub fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position as usize)
    }
}

pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            tokenizer: Tokenizer::new(input),
        }
    }

    pub fn get_next_token(&mut self) -> Option<String> {
        let mut s = String::new();
        let mut is_quoted = false;

        if self.tokenizer.peek().is_none() {
            return None;
        }

        while let Some(c) = self.tokenizer.next() {
            if c.is_whitespace() && !is_quoted {
                break;
            }

            if c == '"' && !is_quoted {
                is_quoted = true;
                continue;
            }

            if c == '"' && is_quoted {
                break;
            }

            if is_quoted && c == '\n' {
                panic!("Unexpected newline");
            }

            s.push(c);
        }

        Some(s)
    }

    pub fn parse(&mut self) -> Vec<String> {
        let mut tokens = Vec::new();

        loop {
            let token = self.get_next_token();

            if token.is_none() {
                break;
            }

            tokens.push(token.unwrap());
        }

        tokens
    }
}

pub trait CommandHandler {
    fn handle(&self, db: &mut MemoryDatabase, args: Vec<String>) -> String;
}

pub struct SetCommandHandler {}

impl CommandHandler for SetCommandHandler {
    fn handle(&self, db: &mut MemoryDatabase, args: Vec<String>) -> String {
        if args.len() != 2 {
            return "-ERR wrong number of arguments for 'set' command".to_string();
        }

        match db.set(args[0].clone(), Value::String(args[1].clone())) {
            Ok(_) => "+OK".to_string(),
            Err(_) => "-ERR an error occurred".to_string(),
        }
    }
}

pub fn dispatch(db: &mut MemoryDatabase, args: Vec<String>) -> String {
    let command = args[0].as_str();

    match command {
        "set" => SetCommandHandler {}.handle(db, args[1..].to_vec()),
        _ => "-ERR unknown command".to_string(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenizer_next() {
        let mut parser = Parser::new("set key \"string value\"".to_string());
        let result = parser.parse();

        assert_eq!(result, vec!["set", "key", "string value"]);
    }
}
