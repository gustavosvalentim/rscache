#[derive(PartialEq, Eq)]
enum Token {
    String(String),
    Array(usize),
}

struct Lexer<'a> {
    input: Box<dyn Iterator<Item = char> + 'a>,
}

impl<'a> Lexer<'a> {
    pub fn new<I>(input: I) -> Self
    where
        I: IntoIterator<Item = char> + 'a,
    {
        Self {
            input: Box::new(input.into_iter()),
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
                        let c = self.input.next()?;
                        s.push(c);
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
                let mut iter = self.input.as_mut().peekable();

                // TODO: Not sure why but this peek is moving the iterator cursor.
                // Need to find a way to improve this
                if let Some(n) = iter.peek() {
                    if *n == '\n' {
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
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input.chars()),
        }
    }

    pub fn parse(&mut self) -> Vec<String> {
        let mut tokens = Vec::new();

        while let Some(token) = self.lexer.next() {
            match token {
                Token::String(s) => tokens.push(s),
                Token::Array(size) => {
                    let inner_tokens = self.parse();

                    if inner_tokens.iter().count() != size {
                        panic!("Array size mismatch");
                    }

                    inner_tokens.iter().for_each(|t| tokens.push(t.to_string()));
                }
            }
        }

        tokens
    }
}
