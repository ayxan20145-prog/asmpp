use clap::Parser as ClapParser;
use std::fs;

#[derive(ClapParser, Debug)]
#[command(
    name = "asmpp",
    version,
    about = "a higher level assembly language that compiles to assembly"
)]
struct Cli {
    input: String,
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Register(Register),
    Name(String),
    Value(i64),

    Equals,

    Syscall,

    Fn,

    LBrace,
    RBrace,

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
    Mov(Register, Value),
    Syscall,
    Function(String, Vec<Statement>),
}

#[derive(Debug)]
enum Value {
    Register(Register),
    Number(i64),
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

            Some('{') => {
                self.advance();
                Token::LBrace
            }

            Some('}') => {
                self.advance();
                Token::RBrace
            }

            Some(c) if c.is_ascii_digit() => {
                let mut value = String::new();

                loop {
                    match self.current() {
                        Some(c) => {
                            if c.is_ascii_digit() {
                                value.push(c);
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        None => break,
                    }
                }

                Token::Value(value.parse().unwrap())
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

                if name == "syscall" {
                    Token::Syscall
                } else if name == "fn" {
                    Token::Fn
                } else {
                    match name.as_str() {
                        "rax" => Token::Register(Register::RAX),
                        "rbx" => Token::Register(Register::RBX),
                        "rcx" => Token::Register(Register::RCX),
                        "rdx" => Token::Register(Register::RDX),
                        "rsi" => Token::Register(Register::RSI),
                        "rdi" => Token::Register(Register::RDI),
                        "rbp" => Token::Register(Register::RBP),
                        "rsp" => Token::Register(Register::RSP),
                        _ => Token::Name(name),
                    }
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
    fn current(&self) -> Token {
        self.tokens[self.position].clone()
    }
    fn advance(&mut self) -> Token {
        let token = self.current();
        self.position += 1;
        token
    }
    fn parse_statement(&mut self) -> Statement {
        match self.current() {
            Token::Syscall => {
                self.advance();

                match self.current() {
                    Token::Semicolon => {
                        self.advance();
                    }
                    _ => panic!("expected ';'"),
                }

                Statement::Syscall
            }
            Token::Fn => {
                self.advance();

                let name = match self.advance() {
                    Token::Name(name) => name,
                    _ => panic!("expected function name"),
                };

                match self.advance() {
                    Token::LBrace => {}
                    _ => panic!("expected '{{'"),
                }

                let mut statements = Vec::new();

                while self.current() != Token::RBrace {
                    if self.current() == Token::Eof {
                        panic!("unclosed function");
                    }

                    statements.push(self.parse_statement());
                }

                self.advance();

                Statement::Function(name, statements)
            }
            _ => {
                let reg = match self.advance() {
                    Token::Register(reg) => reg,
                    _ => panic!("expected register"),
                };

                match self.advance() {
                    Token::Equals => {}
                    _ => panic!("expected '='"),
                }

                let value = match self.advance() {
                    Token::Register(reg) => Value::Register(reg),
                    Token::Value(val) => Value::Number(val),
                    _ => panic!("expected value"),
                };

                match self.current() {
                    Token::Semicolon => self.advance(),
                    _ => panic!("expected ';'"),
                };

                Statement::Mov(reg, value)
            }
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
    let args = Cli::parse();

    let content = fs::read_to_string(&args.input).expect("Failed to read program");

    let mut lexer = Lexer::new(&content);

    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);

    let program = parser.parse_program();

    let assembly = compile(&program);

    fs::write("program.asm", assembly).expect("Failed to write program");
}
fn register_name(register: &Register) -> &'static str {
    match register {
        Register::RAX => "rax",
        Register::RBX => "rbx",
        Register::RCX => "rcx",
        Register::RDX => "rdx",
        Register::RSI => "rsi",
        Register::RDI => "rdi",
        Register::RBP => "rbp",
        Register::RSP => "rsp",
    }
}
fn compile(program: &Program) -> String {
    let mut assembly = String::new();

    assembly.push_str("section .text\nglobal _start\n\n_start:\n");

    for statement in &program.statements {
        match statement {
            Statement::Mov(reg, value) => {
                assembly.push_str(&format!("    mov {}, ", register_name(reg)));

                match value {
                    Value::Register(reg) => assembly.push_str(register_name(reg)),
                    Value::Number(num) => assembly.push_str(&num.to_string()),
                }

                assembly.push_str("\n");
            }
            Statement::Syscall => {
                assembly.push_str("    syscall\n");
            }
            Statement::Function(name, statements) => {
                assembly.push_str(&format!("{}:\n", name));

                for statement in statements {
                    match statement {
                        Statement::Mov(reg, value) => {
                            assembly.push_str(&format!("    mov {}, ", register_name(reg)));

                            match value {
                                Value::Register(reg) => assembly.push_str(register_name(reg)),
                                Value::Number(num) => assembly.push_str(&num.to_string()),
                            }

                            assembly.push_str("\n");
                        }
                        Statement::Syscall => {
                            assembly.push_str("    syscall\n");
                        }
                        _ => panic!("error message"),
                    }
                }
                assembly.push('\n');
            }
        }
    }
    assembly
}
