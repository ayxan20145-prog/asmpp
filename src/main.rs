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
    fn new() -> Self {
        Self {
            source: Vec::new(),
            position: 0,
        }
    }
}

fn main() {
    let lexer = Lexer::new();
}
