#[derive(Debug, PartialEq)]
enum Token {
    Register(Register),
    Mov,

    Equals,

    Semicolon,
    Eof,
}

#[derive(Debug, PartialEq)]
enum Register {
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RBP,
    RSP,
}

struct Lexer {
    source: Vec<char>,
    position: usize,
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Lexer {
    fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
        }
    }
    fn current(&self) -> Option<char> {
        self.source.get(self.position).copied()
    }
    fn advance(&mut self) {
        self.position += 1;
    }
    fn next_token(&mut self) -> Token {
        loop {
            match self.current() {
                Some(c) => {
                    if c.is_whitespace() {
                        self.advance();
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }

        match self.current() {
            None => Token::Eof,

            Some(';') => {
                self.advance();
                Token::Semicolon
            }

            Some('=') => {
                self.advance();
                Token::Equals
            }

            Some(c) if c.is_alphabetic() => {
                let mut name = String::new();

                loop {
                    match self.current() {
                        Some(c) => {
                            if c.is_alphanumeric() {
                                name.push(c);
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        None => break,
                    }
                }

                if name == "mov" {
                    Token::Mov
                } else {
                    Token::Register(match name.as_str() {
                        "rax" => Register::RAX,
                        "rbx" => Register::RBX,
                        "rcx" => Register::RCX,
                        "rdx" => Register::RDX,
                        "rsi" => Register::RSI,
                        "rdi" => Register::RDI,
                        "rbp" => Register::RBP,
                        "rsp" => Register::RSP,
                        _ => panic!("error message"),
                    })
                }
            }
            Some(c) => panic!("unexpected char: {}", c),
        }
    }
    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();

            if token == Token::Eof {
                tokens.push(Token::Eof);
                break;
            }

            tokens.push(token);
        }

        tokens
    }
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }
}

fn main() {
    let mut lexer = Lexer::new("rax = rbx");

    let tokens = lexer.tokenize();

    let parser = Parser::new(tokens);
}
