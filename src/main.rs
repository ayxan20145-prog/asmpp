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

        match self.current() {
            Token::Semicolon => self.advance(),
            _ => panic!("expected ';'"),
        };

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
    let args = Cli::parse();

    let content = fs::read_to_string(&args.input).expect("Failed to read program");

    let mut lexer = Lexer::new(&content);

    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);

    let program = parser.parse_program();

    let assembly = compile(&program);

    fs::write("program.asm", assembly).expect("Failed to write program");
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

    assembly.push_str("section .text\n");
    assembly.push_str("global _start\n\n");
    assembly.push_str("_start:\n");

    for statement in &program.statements {
        match statement {
            Statement::Mov(reg1, reg2) => {
                assembly.push_str(&format!(
                    "    mov {}, {}\n",
                    register_name(reg1),
                    register_name(reg2)
                ));
            }
        }
    }

    assembly
}
