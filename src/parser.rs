use std::rc::Rc;

use lexer::Lexer;
use lexer::Token;

struct Parser {
    source: &String,

    lexer: &Lexer,
    tokens: Iter<Token>,
}

impl Parser {
    pub fn new(source: &String, lexer: &Lexer) -> Parser {
        Parser {
            source,
            lexer,
            tokens: lexer.tokens.peekable(),
        }
    }

    pub fn parse_scope(&mut self, scope: &Rc<Ast>) {
        loop {
            let token = self.peek_token();
            if TokenKind::End == token.kind {
                // NOTE(ydolev): Parsed all tokens.
                break;
            }
        }
    }

    fn peek_token(&self) -> &Token {
        self.tokens.peek().unwrap()
    }

    fn eat_token(&mut self) -> &Token {
        self.tokens.next().unwrap()
    }
}
