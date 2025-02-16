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

        while let Some(c) = self.tokenizer.peek() {
            // TODO: figure out a better way to handle this to try and avoid
            // the need of doing `self.tokenizer.next()` all the time.

            if c == '\r' {
                if s.chars().count() > 0 {
                    break;
                }

                self.skip(1);

                if self.tokenizer.peek() == Some('\n') {
                    self.tokenizer.next();
                    s.push_str("\r\n");
                    break;
                }
            }

            if c == ' ' && !is_quoted {
                if s.chars().count() > 0 {
                    break;
                }

                s.push(c);
                self.tokenizer.next();
                break;
            }

            if c == '"' && !is_quoted {
                is_quoted = true;
                self.tokenizer.next();
                continue;
            }

            if c == '"' && is_quoted {
                self.tokenizer.next();
                break;
            }

            s.push(c);
            self.tokenizer.next();
        }

        Some(s)
    }

    fn skip(&mut self, n: i32) {
        for _ in 0..n {
            self.tokenizer.next();
        }
    }

    pub fn parse(&mut self) -> Vec<String> {
        let mut token_count = 0;
        let mut tokens = Vec::new();

        loop {
            let token = self.get_next_token();

            if token.is_none() {
                break;
            }

            if let Some(token) = token {
                if token.is_empty() {
                    continue;
                }

                if token.starts_with("*") {
                    token_count = token[1..].parse::<i32>().unwrap();

                    // Next character is a newline
                    // So we skip it
                    self.skip(2);
                } else if token.starts_with("$") {
                    let token_size = token[1..].parse::<i32>().unwrap();
                    let mut next_token = String::new();
                    let mut cur_token_size = 0;

                    // Next character is a newline
                    // So we skip it
                    self.skip(2);

                    loop {
                        let token = self.get_next_token();
                        cur_token_size += token.as_ref().unwrap().chars().count();

                        if cur_token_size as i32 > token_size {
                            break;
                        }

                        next_token.push_str(token.unwrap().as_ref());
                    }

                    tokens.push(next_token);
                } else {
                    if token == "\r\n" || token == " " {
                        continue;
                    }

                    tokens.push(token);
                }
            }
        }

        if token_count > 0 && tokens.iter().count() as i32 != token_count {
            panic!(
                "Expected {} tokens, found {}",
                token_count,
                tokens.iter().count()
            );
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
        if args.len() < 3 {
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

    match command.to_lowercase().as_ref() {
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
