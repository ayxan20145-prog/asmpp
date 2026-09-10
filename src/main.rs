enum Token {
    Register(Register),
}

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
}

fn main() {
    let mut lexer = Lexer::new("mov rax, 1");
    println!("{}", lexer.current().unwrap());
    lexer.advance();
    println!("{}", lexer.current().unwrap());
}
