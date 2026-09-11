#[derive(Debug, PartialEq, Clone)]
enum Token {
    Register(Register),

    Equals,

    Semicolon,
    Eof,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug)]
enum Statement {
    Mov(Register, Register),
}

struct Lexer {
    source: Vec<char>,
    position: usize,
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

#[derive(Debug)]
struct Program {
    statements: Vec<Statement>,
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

                Token::Register(parse_register(name.as_str()))
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
    fn current(&self) -> Token {
        self.tokens[self.position].clone()
    }
    fn advance(&mut self) -> Token {
        let token = self.current();
        self.position += 1;
        token
    }
    fn parse_statement(&mut self) -> Statement {
        let reg1 = self.advance();
        self.advance();
        let reg2 = self.advance();

        match (reg1, reg2) {
            (Token::Register(reg1), Token::Register(reg2)) => Statement::Mov(reg1, reg2),
            _ => panic!("expected registers"),
        }
    }
    fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();

        while self.current() != Token::Eof {
            statements.push(self.parse_statement());
        }

        Program { statements }
    }
}

fn main() {
    let mut lexer = Lexer::new("rax = rbx");

    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);

    let program = parser.parse_program();

    println!("{:#?}", program);
}
fn parse_register(name: &str) -> Register {
    match name {
        "rax" => Register::RAX,
        "rbx" => Register::RBX,
        "rcx" => Register::RCX,
        "rdx" => Register::RDX,
        "rsi" => Register::RSI,
        "rdi" => Register::RDI,
        "rbp" => Register::RBP,
        "rsp" => Register::RSP,
        _ => panic!("unknown register"),
    }
}
