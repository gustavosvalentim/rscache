use std::str::Chars;

enum Token {
    String(String),
    Array(usize),
}

struct Lexer<'a> {
    input: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input: input.chars(),
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        enum CurrToken {
            SimpleString,
            QuotedString,
            BulkString,
            Array,
        }

        loop {
            let c = self.input.next()?;

            let curr_token = match c {
                '*' => CurrToken::Array,
                '$' => CurrToken::BulkString,
                'a'..='z' | 'A'..='Z' => CurrToken::SimpleString,
                '"' => CurrToken::QuotedString,
                _ => continue,
            };

            break match curr_token {
                CurrToken::Array => {
                    let size = self.read_size();

                    Some(Token::Array(size))
                }
                CurrToken::BulkString => {
                    let size = self.read_size();
                    let mut s = String::new();

                    for _ in 0..size {
                        s.push(self.input.next()?);
                    }

                    Some(Token::String(s))
                }
                CurrToken::SimpleString => {
                    let mut s = String::new();

                    s.push(c);

                    loop {
                        let c = self.input.next()?;

                        if c.is_whitespace() {
                            break;
                        }

                        s.push(c);
                    }

                    Some(Token::String(s))
                }
                CurrToken::QuotedString => {
                    let mut s = String::new();

                    loop {
                        let c = self.input.next()?;

                        if c == '"' {
                            break;
                        }

                        s.push(c);
                    }

                    Some(Token::String(s))
                }
            };
        }
    }

    fn read_size(&mut self) -> usize {
        let mut size = String::new();

        while let Some(c) = self.input.next() {
            if c.is_digit(10) {
                size.push(c);
            } else if c == '\r' {
                let iter = self.input.clone();
                let mut peekable = iter.peekable();

                if let Some(n) = peekable.peek() {
                    if *n == '\n' {
                        self.input.next();
                        break;
                    }
                }
            }
        }

        size.parse().unwrap()
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser {
            lexer: Lexer::new(input),
        }
    }

    pub fn parse(&mut self) -> Vec<String> {
        let mut tokens = Vec::new();

        while let Some(token) = self.lexer.next() {
            match token {
                Token::String(s) => tokens.push(s),
                Token::Array(size) => {
                    tokens = self.parse();

                    if tokens.iter().count() != size {
                        panic!("Array size mismatch");
                    }
                }
            }
        }

        tokens
    }
}
