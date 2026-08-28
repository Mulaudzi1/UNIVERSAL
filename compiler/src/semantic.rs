use std::collections::HashMap;
use crate::{ast::*, diagnostic::Diagnostic, types::{self, Type}};

pub struct Analyzer { entities: HashMap<String, EntityDecl>, vars: HashMap<String, Type>, functions: HashMap<String, FunctionDecl>, errors: Vec<Diagnostic> }
impl Analyzer {
    pub fn new()->Self{Self{entities:HashMap::new(),vars:HashMap::new(),functions:HashMap::new(),errors:Vec::new()}}
    pub fn analyze(mut self,p:&Program)->Result<(),Vec<Diagnostic>>{
        for s in &p.items { match s { Stmt::Entity(e)=>{if self.entities.insert(e.name.clone(),e.clone()).is_some(){self.errors.push(Diagnostic::error("U3000",format!("Entity '{}' is already declared.",e.name),Some(e.span)));}}, Stmt::Function(f)=>{self.functions.insert(f.name.clone(),f.clone());}, _=>{} } }
        for e in self.entities.values() { for p in &e.properties { if types::builtin(&p.ty.name).is_none() && !self.entities.contains_key(&p.ty.name) { self.errors.push(Diagnostic::error("U3001",format!("Unknown type '{}'.",p.ty.name),Some(p.span))); } } }
        for s in &p.items { self.check_stmt(s); }
        if self.errors.is_empty(){Ok(())}else{Err(self.errors)}
    }
    fn check_stmt(&mut self,s:&Stmt){match s{
        Stmt::Let{name,value,..}=>{let ty=self.infer(value); self.vars.insert(name.clone(),ty);}
        Stmt::When{branches,otherwise,..}=>{for(c,b) in branches{let t=self.infer(c); if t!=Type::Boolean && t!=Type::Unknown{self.errors.push(Diagnostic::error("U3002","WHEN condition must be Boolean.",Some(c.span())));} for s in b{self.check_stmt(s);}} for s in otherwise{self.check_stmt(s);}}
        Stmt::Print{value,..}|Stmt::Validate{message:value,..}|Stmt::Return{value,..}|Stmt::Expr{expr:value,..}=>{self.infer(value);}
        Stmt::Function(f)=>{let old=self.vars.clone(); for p in &f.params { let t=p.ty.as_ref().map(|t|self.resolve_type(t,false)).unwrap_or(Type::Unknown); self.vars.insert(p.name.clone(),t);} for s in &f.body{self.check_stmt(s);} self.vars=old;}
        _=>{}
    }}
    fn resolve_type(&self,t:&TypeRef,opt:bool)->Type{let base=types::builtin(&t.name).or_else(||self.entities.contains_key(&t.name).then(||Type::Entity(t.name.clone()))).unwrap_or(Type::Unknown); if opt{Type::Optional(Box::new(base))}else{base}}
    fn infer(&mut self,e:&Expr)->Type{match e{
        Expr::String(..)=>Type::Text, Expr::Number(n,..)=>if n.contains('.') {Type::Decimal}else{Type::Number}, Expr::Boolean(..)=>Type::Boolean, Expr::Null(..)=>Type::Optional(Box::new(Type::Unknown)),
        Expr::Identifier(n,s)=>self.vars.get(n).cloned().unwrap_or_else(||{self.errors.push(Diagnostic::error("U3003",format!("Unknown name '{n}'."),Some(*s)));Type::Unknown}),
        Expr::Call{callee,args,span}=>{if let Some(ent)=self.entities.get(callee).cloned(){for(named,val) in args{if let Some(n)=named{if let Some(prop)=ent.properties.iter().find(|p|p.name==*n){let got=self.infer(val); let want=self.resolve_type(&prop.ty,prop.optional); if !compatible(&got,&want){self.errors.push(Diagnostic::error("U3004",format!("Property '{}.{}' expects {:?}, got {:?}.",callee,n,want,got),Some(val.span())));}}else{self.errors.push(Diagnostic::error("U3005",format!("Entity '{}' has no property '{}'.",callee,n),Some(val.span())));}}} Type::Entity(callee.clone())} else if let Some(f)=self.functions.get(callee){f.return_type.as_ref().map(|t|self.resolve_type(t,false)).unwrap_or(Type::Void)} else {self.errors.push(Diagnostic::error("U3006",format!("Unknown callable '{callee}'."),Some(*span)));Type::Unknown}},
        Expr::Property{object,name,span}=>{let ot=self.infer(object); match unwrap_optional(ot){Type::Entity(en)=>{if let Some(ent)=self.entities.get(&en){if let Some(p)=ent.properties.iter().find(|p|p.name==*name){self.resolve_type(&p.ty,p.optional)}else{self.errors.push(Diagnostic::error("U3007",format!("Entity '{}' has no property '{}'.",en,name),Some(*span)).with_help(format!("Check the ENTITY {} declaration.",en)));Type::Unknown}}else{Type::Unknown}},other=>{self.errors.push(Diagnostic::error("U3008",format!("Cannot access property '{}' on {:?}.",name,other),Some(*span)));Type::Unknown}}},
        Expr::Has{subject,property,span,..}|Expr::IsProperty{subject,property,span,..}=>{let st=self.infer(subject); if let Type::Entity(en)=unwrap_optional(st){if let Some(ent)=self.entities.get(&en){if !ent.properties.iter().any(|p|p.name==*property){self.errors.push(Diagnostic::error("U3009",format!("Entity '{}' has no property '{}'.",en,property),Some(*span)));}}} Type::Boolean},
        Expr::Exists{expr,..}=>{self.infer(expr);Type::Boolean}, Expr::Unary{op:_,expr,..}=>{self.infer(expr);Type::Boolean},
        Expr::Binary{left,op,right,..}=>{let l=self.infer(left);let r=self.infer(right);match op{BinaryOp::And|BinaryOp::Or|BinaryOp::Eq|BinaryOp::NotEq|BinaryOp::Greater|BinaryOp::GreaterEq|BinaryOp::Less|BinaryOp::LessEq=>Type::Boolean,_=>if l==Type::Decimal||r==Type::Decimal{Type::Decimal}else{Type::Number}}}
    }}
}
fn unwrap_optional(t:Type)->Type{if let Type::Optional(x)=t{*x}else{t}}
fn compatible(a:&Type,b:&Type)->bool{a==b||*a==Type::Unknown||*b==Type::Unknown||matches!((a,b),(Type::Number,Type::Decimal)|(Type::Optional(_),Type::Optional(_)))}
