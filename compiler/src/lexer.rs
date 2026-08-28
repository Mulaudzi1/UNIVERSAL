use crate::{diagnostic::Diagnostic, token::{Span, Token, TokenKind}};

pub struct Lexer<'a> {
    src: &'a str,
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src, chars: src.chars().collect(), pos: 0, line: 1, col: 1 }
    }

    pub fn lex(mut self) -> Result<Vec<Token>, Vec<Diagnostic>> {
        let mut out = Vec::new();
        let mut errors = Vec::new();
        while let Some(c) = self.peek() {
            let start = self.span_start();
            match c {
                ' ' | '\t' | '\r' => { self.bump(); }
                '\n' => { self.bump(); out.push(Token { kind: TokenKind::Newline, span: self.finish_span(start) }); }
                '#' => { self.skip_comment(); }
                '/' if self.peek_n(1) == Some('/') => { self.bump(); self.bump(); self.skip_comment(); }
                '"' => match self.lex_string(start) { Ok(t) => out.push(t), Err(e) => errors.push(e) },
                '0'..='9' => out.push(self.lex_number(start)),
                'A'..='Z' | 'a'..='z' | '_' => out.push(self.lex_ident(start)),
                '=' => { self.bump(); if self.peek() == Some('=') { self.bump(); out.push(self.tok(TokenKind::EqEq, start)); } else { out.push(self.tok(TokenKind::Equal, start)); } }
                '!' => { self.bump(); if self.peek() == Some('=') { self.bump(); out.push(self.tok(TokenKind::NotEq, start)); } else { errors.push(Diagnostic::error("U1001", "Unexpected '!'. Use NOT or !=.", Some(self.finish_span(start)))); } }
                '>' => { self.bump(); if self.peek() == Some('=') { self.bump(); out.push(self.tok(TokenKind::GreaterEq, start)); } else { out.push(self.tok(TokenKind::Greater, start)); } }
                '<' => { self.bump(); if self.peek() == Some('=') { self.bump(); out.push(self.tok(TokenKind::LessEq, start)); } else { out.push(self.tok(TokenKind::Less, start)); } }
                '-' => { self.bump(); if self.peek() == Some('>') { self.bump(); out.push(self.tok(TokenKind::Arrow, start)); } else { out.push(self.tok(TokenKind::Minus, start)); } }
                ':' => { self.bump(); out.push(self.tok(TokenKind::Colon, start)); }
                ',' => { self.bump(); out.push(self.tok(TokenKind::Comma, start)); }
                '.' => { self.bump(); out.push(self.tok(TokenKind::Dot, start)); }
                '(' => { self.bump(); out.push(self.tok(TokenKind::LParen, start)); }
                ')' => { self.bump(); out.push(self.tok(TokenKind::RParen, start)); }
                '?' => { self.bump(); out.push(self.tok(TokenKind::Question, start)); }
                '+' => { self.bump(); out.push(self.tok(TokenKind::Plus, start)); }
                '*' => { self.bump(); out.push(self.tok(TokenKind::Star, start)); }
                '/' => { self.bump(); out.push(self.tok(TokenKind::Slash, start)); }
                _ => { self.bump(); errors.push(Diagnostic::error("U1000", format!("Unexpected character '{c}'."), Some(self.finish_span(start)))); }
            }
        }
        out.push(Token { kind: TokenKind::Eof, span: Span { start: self.src.len(), end: self.src.len(), line: self.line, column: self.col } });
        if errors.is_empty() { Ok(out) } else { Err(errors) }
    }

    fn lex_ident(&mut self, start: (usize, usize, usize)) -> Token {
        let begin = self.pos;
        while matches!(self.peek(), Some(c) if c.is_ascii_alphanumeric() || c == '_') { self.bump(); }
        let s: String = self.chars[begin..self.pos].iter().collect();
        let upper = s.to_ascii_uppercase();
        let kind = match upper.as_str() {
            "ENTITY" => TokenKind::Entity, "FUNCTION" => TokenKind::Function, "RETURN" => TokenKind::Return,
            "WHEN" => TokenKind::When, "ELSE" => TokenKind::Else, "OTHERWISE" => TokenKind::Otherwise, "END" => TokenKind::End,
            "AND" => TokenKind::And, "OR" => TokenKind::Or, "NOT" => TokenKind::Not, "HAS" => TokenKind::Has,
            "DOES" => TokenKind::Does, "HAVE" => TokenKind::Have, "IS" => TokenKind::Is, "EXISTS" => TokenKind::Exists,
            "PRINT" => TokenKind::Print, "VALIDATE" => TokenKind::Validate, "TRUE" => TokenKind::True, "FALSE" => TokenKind::False,
            "NULL" => TokenKind::Null, _ => TokenKind::Identifier(s),
        };
        Token { kind, span: self.finish_span(start) }
    }

    fn lex_number(&mut self, start: (usize, usize, usize)) -> Token {
        let begin = self.pos;
        while matches!(self.peek(), Some(c) if c.is_ascii_digit()) { self.bump(); }
        if self.peek() == Some('.') && matches!(self.peek_n(1), Some(c) if c.is_ascii_digit()) {
            self.bump();
            while matches!(self.peek(), Some(c) if c.is_ascii_digit()) { self.bump(); }
        }
        let s: String = self.chars[begin..self.pos].iter().collect();
        Token { kind: TokenKind::Number(s), span: self.finish_span(start) }
    }

    fn lex_string(&mut self, start: (usize, usize, usize)) -> Result<Token, Diagnostic> {
        self.bump();
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c == '"' { self.bump(); return Ok(Token { kind: TokenKind::String(s), span: self.finish_span(start) }); }
            if c == '\n' { return Err(Diagnostic::error("U1002", "Unterminated string literal.", Some(self.finish_span(start)))); }
            if c == '\\' {
                self.bump();
                match self.peek() {
                    Some('n') => { self.bump(); s.push('\n'); }, Some('t') => { self.bump(); s.push('\t'); },
                    Some('"') => { self.bump(); s.push('"'); }, Some('\\') => { self.bump(); s.push('\\'); },
                    Some(other) => { self.bump(); s.push(other); }, None => break,
                }
            } else { self.bump(); s.push(c); }
        }
        Err(Diagnostic::error("U1002", "Unterminated string literal.", Some(self.finish_span(start))))
    }

    fn skip_comment(&mut self) { while !matches!(self.peek(), None | Some('\n')) { self.bump(); } }
    fn peek(&self) -> Option<char> { self.chars.get(self.pos).copied() }
    fn peek_n(&self, n: usize) -> Option<char> { self.chars.get(self.pos + n).copied() }
    fn bump(&mut self) { if let Some(c) = self.peek() { self.pos += 1; if c == '\n' { self.line += 1; self.col = 1; } else { self.col += 1; } } }
    fn span_start(&self) -> (usize, usize, usize) { (self.pos, self.line, self.col) }
    fn finish_span(&self, s: (usize, usize, usize)) -> Span { Span { start: s.0, end: self.pos, line: s.1, column: s.2 } }
    fn tok(&self, kind: TokenKind, s: (usize, usize, usize)) -> Token { Token { kind, span: self.finish_span(s) } }
}
