use crate::token::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Program { pub items: Vec<Stmt> }

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Entity(EntityDecl),
    Function(FunctionDecl),
    Let { name: String, value: Expr, span: Span },
    When { branches: Vec<(Expr, Vec<Stmt>)>, otherwise: Vec<Stmt>, span: Span },
    Print { value: Expr, span: Span },
    Validate { message: Expr, span: Span },
    Return { value: Expr, span: Span },
    Action { words: Vec<String>, span: Span },
    Expr { expr: Expr, span: Span },
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntityDecl { pub name: String, pub properties: Vec<PropertyDecl>, pub span: Span }
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyDecl { pub name: String, pub ty: TypeRef, pub optional: bool, pub span: Span }
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl { pub name: String, pub params: Vec<Param>, pub return_type: Option<TypeRef>, pub body: Vec<Stmt>, pub span: Span }
#[derive(Debug, Clone, PartialEq)]
pub struct Param { pub name: String, pub ty: Option<TypeRef>, pub span: Span }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeRef { pub name: String }

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    String(String, Span), Number(String, Span), Boolean(bool, Span), Null(Span), Identifier(String, Span),
    Property { object: Box<Expr>, name: String, span: Span },
    Call { callee: String, args: Vec<(Option<String>, Expr)>, span: Span },
    Binary { left: Box<Expr>, op: BinaryOp, right: Box<Expr>, span: Span },
    Unary { op: UnaryOp, expr: Box<Expr>, span: Span },
    Has { subject: Box<Expr>, property: String, negated: bool, span: Span },
    IsProperty { subject: Box<Expr>, property: String, negated: bool, span: Span },
    Exists { expr: Box<Expr>, span: Span },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp { And, Or, Eq, NotEq, Greater, GreaterEq, Less, LessEq, Add, Sub, Mul, Div }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp { Not, Negate }

impl Expr {
    pub fn span(&self) -> Span { match self {
        Expr::String(_,s)|Expr::Number(_,s)|Expr::Boolean(_,s)|Expr::Null(s)|Expr::Identifier(_,s) => *s,
        Expr::Property{span,..}|Expr::Call{span,..}|Expr::Binary{span,..}|Expr::Unary{span,..}|Expr::Has{span,..}|Expr::IsProperty{span,..}|Expr::Exists{span,..} => *span,
    }}
}
