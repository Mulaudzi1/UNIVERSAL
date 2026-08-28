use crate::{ast::*, diagnostic::Diagnostic, token::{Span, Token, TokenKind}};

pub struct Parser { tokens: Vec<Token>, pos: usize }

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self { Self { tokens, pos: 0 } }

    pub fn parse(mut self) -> Result<Program, Vec<Diagnostic>> {
        let mut items = Vec::new();
        let mut errors = Vec::new();
        self.skip_newlines();
        while !self.at(&TokenKind::Eof) {
            match self.parse_stmt() {
                Ok(s) => items.push(s),
                Err(e) => { errors.push(e); self.recover_line(); }
            }
            self.skip_newlines();
        }
        if errors.is_empty() { Ok(Program { items }) } else { Err(errors) }
    }

    fn parse_stmt(&mut self) -> Result<Stmt, Diagnostic> {
        match self.current().kind.clone() {
            TokenKind::Entity => self.parse_entity(),
            TokenKind::Function => self.parse_function(),
            TokenKind::When => self.parse_when(),
            TokenKind::Print => self.parse_print(),
            TokenKind::Validate => self.parse_validate(),
            TokenKind::Return => self.parse_return(),
            TokenKind::Identifier(_) => self.parse_identifier_stmt(),
            other => Err(self.err("U2000", format!("Expected a statement, found {other:?}."))),
        }
    }

    fn parse_entity(&mut self) -> Result<Stmt, Diagnostic> {
        let span = self.bump().span;
        let name = self.expect_ident("Expected entity name after ENTITY.")?;
        self.expect_newline()?;
        let mut properties = Vec::new();
        self.skip_newlines();
        while !self.at(&TokenKind::End) {
            if self.at(&TokenKind::Eof) { return Err(self.err("U2001", "Unclosed ENTITY block. Add END.")); }
            let pspan = self.current().span;
            let pname = self.expect_ident("Expected property name.")?;
            self.expect_simple(TokenKind::Colon, "Expected ':' after property name.")?;
            let ty = TypeRef { name: self.expect_ident("Expected property type.")? };
            let optional = self.consume(&TokenKind::Question);
            properties.push(PropertyDecl { name: pname, ty, optional, span: pspan });
            self.expect_line_end()?;
            self.skip_newlines();
        }
        self.bump();
        self.consume(&TokenKind::Newline);
        Ok(Stmt::Entity(EntityDecl { name, properties, span }))
    }

    fn parse_function(&mut self) -> Result<Stmt, Diagnostic> {
        let span = self.bump().span;
        let name = self.expect_ident("Expected function name.")?;
        self.expect_simple(TokenKind::LParen, "Expected '(' after function name.")?;
        let mut params = Vec::new();
        if !self.at(&TokenKind::RParen) {
            loop {
                let pspan = self.current().span;
                let pname = self.expect_ident("Expected parameter name.")?;
                let ty = if self.consume(&TokenKind::Colon) { Some(TypeRef { name: self.expect_ident("Expected parameter type.")? }) } else { None };
                params.push(Param { name: pname, ty, span: pspan });
                if !self.consume(&TokenKind::Comma) { break; }
            }
        }
        self.expect_simple(TokenKind::RParen, "Expected ')' after parameters.")?;
        let return_type = if self.consume(&TokenKind::Arrow) { Some(TypeRef { name: self.expect_ident("Expected return type after ->.")? }) } else { None };
        self.expect_newline()?;
        let body = self.parse_block_until(&[TokenKind::End])?;
        self.expect_simple(TokenKind::End, "Expected END after FUNCTION block.")?;
        self.consume(&TokenKind::Newline);
        Ok(Stmt::Function(FunctionDecl { name, params, return_type, body, span }))
    }

    fn parse_when(&mut self) -> Result<Stmt, Diagnostic> {
        let span = self.bump().span;
        let cond = self.parse_expr()?;
        self.expect_newline()?;
        let body = self.parse_block_until(&[TokenKind::Else, TokenKind::Otherwise, TokenKind::End])?;
        let mut branches = vec![(cond, body)];
        while self.at(&TokenKind::Else) {
            self.bump();
            self.expect_simple(TokenKind::When, "ELSE must be followed by WHEN.")?;
            let c = self.parse_expr()?;
            self.expect_newline()?;
            let b = self.parse_block_until(&[TokenKind::Else, TokenKind::Otherwise, TokenKind::End])?;
            branches.push((c, b));
        }
        let otherwise = if self.consume(&TokenKind::Otherwise) {
            self.expect_newline()?;
            self.parse_block_until(&[TokenKind::End])?
        } else { Vec::new() };
        self.expect_simple(TokenKind::End, "Expected END after WHEN block.")?;
        self.consume(&TokenKind::Newline);
        Ok(Stmt::When { branches, otherwise, span })
    }

    fn parse_block_until(&mut self, stops: &[TokenKind]) -> Result<Vec<Stmt>, Diagnostic> {
        let mut out = Vec::new();
        self.skip_newlines();
        while !stops.iter().any(|k| self.at(k)) {
            if self.at(&TokenKind::Eof) { return Err(self.err("U2002", "Unexpected end of file inside block.")); }
            out.push(self.parse_stmt()?);
            self.skip_newlines();
        }
        Ok(out)
    }

    fn parse_print(&mut self) -> Result<Stmt, Diagnostic> { let span=self.bump().span; let value=self.parse_expr()?; self.expect_line_end()?; Ok(Stmt::Print{value,span}) }
    fn parse_validate(&mut self) -> Result<Stmt, Diagnostic> { let span=self.bump().span; let message=self.parse_expr()?; self.expect_line_end()?; Ok(Stmt::Validate{message,span}) }
    fn parse_return(&mut self) -> Result<Stmt, Diagnostic> { let span=self.bump().span; let value=self.parse_expr()?; self.expect_line_end()?; Ok(Stmt::Return{value,span}) }

    fn parse_identifier_stmt(&mut self) -> Result<Stmt, Diagnostic> {
        let span = self.current().span;
        if matches!(self.peek_kind(1), Some(TokenKind::Equal)) {
            let name = self.expect_ident("Expected variable name.")?;
            self.bump();
            let value = self.parse_expr()?;
            self.expect_line_end()?;
            return Ok(Stmt::Let { name, value, span });
        }
        if matches!(self.peek_kind(1), Some(TokenKind::LParen)) {
            let expr = self.parse_expr()?;
            self.expect_line_end()?;
            return Ok(Stmt::Expr { expr, span });
        }
        let mut words = Vec::new();
        while !self.at(&TokenKind::Newline) && !self.at(&TokenKind::Eof) {
            match self.bump().kind {
                TokenKind::Identifier(s) => words.push(s),
                TokenKind::String(s) => words.push(s),
                k => return Err(Diagnostic::error("U2003", format!("Invalid token {k:?} in action phrase."), Some(span))),
            }
        }
        self.consume(&TokenKind::Newline);
        Ok(Stmt::Action { words, span })
    }

    fn parse_expr(&mut self) -> Result<Expr, Diagnostic> { self.parse_or() }
    fn parse_or(&mut self) -> Result<Expr, Diagnostic> {
        let mut e = self.parse_and()?;
        while self.consume(&TokenKind::Or) { let r=self.parse_and()?; let s=e.span(); e=Expr::Binary{left:Box::new(e),op:BinaryOp::Or,right:Box::new(r),span:s}; }
        Ok(e)
    }
    fn parse_and(&mut self) -> Result<Expr, Diagnostic> {
        let mut e = self.parse_comparison()?;
        while self.consume(&TokenKind::And) { let r=self.parse_comparison()?; let s=e.span(); e=Expr::Binary{left:Box::new(e),op:BinaryOp::And,right:Box::new(r),span:s}; }
        Ok(e)
    }
    fn parse_comparison(&mut self) -> Result<Expr, Diagnostic> {
        let mut e = self.parse_additive()?;
        loop {
            let op = if self.consume(&TokenKind::EqEq) {Some(BinaryOp::Eq)} else if self.consume(&TokenKind::NotEq){Some(BinaryOp::NotEq)} else if self.consume(&TokenKind::GreaterEq){Some(BinaryOp::GreaterEq)} else if self.consume(&TokenKind::Greater){Some(BinaryOp::Greater)} else if self.consume(&TokenKind::LessEq){Some(BinaryOp::LessEq)} else if self.consume(&TokenKind::Less){Some(BinaryOp::Less)} else {None};
            if let Some(op)=op { let r=self.parse_additive()?; let s=e.span(); e=Expr::Binary{left:Box::new(e),op,right:Box::new(r),span:s}; } else { break; }
        }
        Ok(e)
    }
    fn parse_additive(&mut self) -> Result<Expr, Diagnostic> {
        let mut e=self.parse_mul()?;
        loop { let op=if self.consume(&TokenKind::Plus){Some(BinaryOp::Add)} else if self.consume(&TokenKind::Minus){Some(BinaryOp::Sub)} else {None}; if let Some(op)=op {let r=self.parse_mul()?; let s=e.span(); e=Expr::Binary{left:Box::new(e),op,right:Box::new(r),span:s};} else {break;} }
        Ok(e)
    }
    fn parse_mul(&mut self) -> Result<Expr, Diagnostic> {
        let mut e=self.parse_unary()?;
        loop { let op=if self.consume(&TokenKind::Star){Some(BinaryOp::Mul)} else if self.consume(&TokenKind::Slash){Some(BinaryOp::Div)} else {None}; if let Some(op)=op {let r=self.parse_unary()?; let s=e.span(); e=Expr::Binary{left:Box::new(e),op,right:Box::new(r),span:s};} else {break;} }
        Ok(e)
    }
    fn parse_unary(&mut self) -> Result<Expr, Diagnostic> {
        if self.consume(&TokenKind::Not) { let span=self.prev().span; let expr=self.parse_unary()?; return Ok(Expr::Unary{op:UnaryOp::Not,expr:Box::new(expr),span}); }
        if self.consume(&TokenKind::Minus) { let span=self.prev().span; let expr=self.parse_unary()?; return Ok(Expr::Unary{op:UnaryOp::Negate,expr:Box::new(expr),span}); }
        self.parse_postfix()
    }
    fn parse_postfix(&mut self) -> Result<Expr, Diagnostic> {
        let mut e=self.parse_primary()?;
        loop {
            if self.consume(&TokenKind::Dot) {
                let name=self.expect_ident("Expected property after '.'.")?; let span=e.span(); e=Expr::Property{object:Box::new(e),name,span};
                if self.consume(&TokenKind::Exists) { let span=e.span(); e=Expr::Exists{expr:Box::new(e),span}; }
                continue;
            }
            if self.consume(&TokenKind::Has) {
                self.consume_article(); let property=self.expect_ident("Expected property name after HAS.")?; let span=e.span(); e=Expr::Has{subject:Box::new(e),property,negated:false,span}; continue;
            }
            if self.consume(&TokenKind::Does) {
                self.expect_simple(TokenKind::Not,"Expected NOT after DOES.")?; self.expect_simple(TokenKind::Have,"Expected HAVE after DOES NOT.")?; self.consume_article(); let property=self.expect_ident("Expected property name after DOES NOT HAVE.")?; let span=e.span(); e=Expr::Has{subject:Box::new(e),property,negated:true,span}; continue;
            }
            if self.consume(&TokenKind::Is) {
                let negated=self.consume(&TokenKind::Not);
                if matches!(self.current().kind, TokenKind::Identifier(_)) { let property=self.expect_ident("Expected state after IS.")?; let span=e.span(); e=Expr::IsProperty{subject:Box::new(e),property,negated,span}; continue; }
                return Err(self.err("U2004","Expected a state name after IS/IS NOT."));
            }
            break;
        }
        Ok(e)
    }
    fn parse_primary(&mut self) -> Result<Expr, Diagnostic> {
        let t=self.bump();
        match t.kind {
            TokenKind::String(s)=>Ok(Expr::String(s,t.span)), TokenKind::Number(n)=>Ok(Expr::Number(n,t.span)), TokenKind::True=>Ok(Expr::Boolean(true,t.span)), TokenKind::False=>Ok(Expr::Boolean(false,t.span)), TokenKind::Null=>Ok(Expr::Null(t.span)),
            TokenKind::Identifier(name)=> {
                if self.consume(&TokenKind::LParen) {
                    let mut args=Vec::new();
                    self.skip_newlines();
                    if !self.at(&TokenKind::RParen) { loop {
                        let named = if matches!(self.current().kind,TokenKind::Identifier(_)) && matches!(self.peek_kind(1),Some(TokenKind::Colon)) { let n=self.expect_ident("Expected argument name.")?; self.bump(); Some(n) } else { None };
                        let value=self.parse_expr()?; args.push((named,value)); self.skip_newlines(); if !self.consume(&TokenKind::Comma){break;} self.skip_newlines();
                    }}
                    self.skip_newlines();
                    self.expect_simple(TokenKind::RParen,"Expected ')' after arguments.")?;
                    Ok(Expr::Call{callee:name,args,span:t.span})
                } else { Ok(Expr::Identifier(name,t.span)) }
            }
            TokenKind::LParen => { let e=self.parse_expr()?; self.expect_simple(TokenKind::RParen,"Expected ')' after expression.")?; Ok(e) }
            other=>Err(Diagnostic::error("U2005",format!("Expected expression, found {other:?}."),Some(t.span)))
        }
    }

    fn consume_article(&mut self) { if let TokenKind::Identifier(s)=&self.current().kind { if s.eq_ignore_ascii_case("a") || s.eq_ignore_ascii_case("an") { self.bump(); } } }
    fn expect_ident(&mut self,msg:&str)->Result<String,Diagnostic>{ let t=self.bump(); if let TokenKind::Identifier(s)=t.kind {Ok(s)} else {Err(Diagnostic::error("U2006",msg,Some(t.span)))} }
    fn expect_newline(&mut self)->Result<(),Diagnostic>{ if self.consume(&TokenKind::Newline){Ok(())} else {Err(self.err("U2007","Expected end of line."))} }
    fn expect_line_end(&mut self)->Result<(),Diagnostic>{ if self.consume(&TokenKind::Newline)||self.at(&TokenKind::Eof)||self.at(&TokenKind::End)||self.at(&TokenKind::Else)||self.at(&TokenKind::Otherwise){Ok(())} else {Err(self.err("U2008","Expected end of line after statement."))} }
    fn expect_simple(&mut self,k:TokenKind,msg:&str)->Result<(),Diagnostic>{ if self.consume(&k){Ok(())} else {Err(self.err("U2009",msg))} }
    fn skip_newlines(&mut self){while self.consume(&TokenKind::Newline){}}
    fn recover_line(&mut self){while !self.at(&TokenKind::Newline)&&!self.at(&TokenKind::Eof){self.bump();} self.consume(&TokenKind::Newline);}
    fn at(&self,k:&TokenKind)->bool{ std::mem::discriminant(&self.current().kind)==std::mem::discriminant(k) }
    fn consume(&mut self,k:&TokenKind)->bool{if self.at(k){self.pos+=1;true}else{false}}
    fn current(&self)->&Token{&self.tokens[self.pos.min(self.tokens.len()-1)]}
    fn prev(&self)->&Token{&self.tokens[self.pos-1]}
    fn peek_kind(&self,n:usize)->Option<&TokenKind>{self.tokens.get(self.pos+n).map(|t|&t.kind)}
    fn bump(&mut self)->Token{let t=self.current().clone(); if !self.at(&TokenKind::Eof){self.pos+=1;} t}
    fn err(&self,code:&'static str,msg:impl Into<String>)->Diagnostic{Diagnostic::error(code,msg,Some(self.current().span))}
}
