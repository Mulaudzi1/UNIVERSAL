use std::collections::{BTreeMap, HashMap};
use crate::{ast::*, diagnostic::Diagnostic};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Text(String), Number(f64), Boolean(bool), Null,
    Entity { type_name: String, fields: BTreeMap<String, Value> },
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RuntimeOutput { pub stdout: Vec<String>, pub validations: Vec<String>, pub actions: Vec<String> }

enum Flow { Continue, Return(Value) }

pub struct Interpreter {
    vars: HashMap<String, Value>,
    entities: HashMap<String, EntityDecl>,
    functions: HashMap<String, FunctionDecl>,
    output: RuntimeOutput,
}

impl Interpreter {
    pub fn new() -> Self { Self { vars: HashMap::new(), entities: HashMap::new(), functions: HashMap::new(), output: RuntimeOutput::default() } }
    pub fn run(mut self, p: &Program) -> Result<RuntimeOutput, Vec<Diagnostic>> {
        for s in &p.items { match s { Stmt::Entity(e)=>{self.entities.insert(e.name.clone(),e.clone());}, Stmt::Function(f)=>{self.functions.insert(f.name.clone(),f.clone());}, _=>{} } }
        for s in &p.items { if !matches!(s,Stmt::Entity(_) | Stmt::Function(_)) { if let Err(e)=self.exec_stmt(s){return Err(vec![e]);} } }
        Ok(self.output)
    }

    fn exec_stmt(&mut self,s:&Stmt)->Result<Flow,Diagnostic>{match s{
        Stmt::Let{name,value,..}=>{let v=self.eval(value)?;self.vars.insert(name.clone(),v);Ok(Flow::Continue)}
        Stmt::When{branches,otherwise,..}=>{for(c,b) in branches{if truthy(&self.eval(c)?)?{return self.exec_block(b);}}self.exec_block(otherwise)}
        Stmt::Print{value,..}=>{let v=self.eval(value)?;self.output.stdout.push(display(&v));Ok(Flow::Continue)}
        Stmt::Validate{message,..}=>{let v=self.eval(message)?;self.output.validations.push(display(&v));Ok(Flow::Continue)}
        Stmt::Return{value,..}=>Ok(Flow::Return(self.eval(value)?)),
        Stmt::Action{words,..}=>{self.output.actions.push(words.join(" "));Ok(Flow::Continue)}
        Stmt::Expr{expr,..}=>{self.eval(expr)?;Ok(Flow::Continue)}
        Stmt::Entity(_)|Stmt::Function(_)=>Ok(Flow::Continue)
    }}
    fn exec_block(&mut self,b:&[Stmt])->Result<Flow,Diagnostic>{for s in b{if let Flow::Return(v)=self.exec_stmt(s)?{return Ok(Flow::Return(v));}}Ok(Flow::Continue)}

    fn eval(&mut self,e:&Expr)->Result<Value,Diagnostic>{match e{
        Expr::String(s,..)=>Ok(Value::Text(s.clone())), Expr::Number(n,s)=>n.parse::<f64>().map(Value::Number).map_err(|_|Diagnostic::error("U4000","Invalid number literal.",Some(*s))), Expr::Boolean(b,..)=>Ok(Value::Boolean(*b)), Expr::Null(..)=>Ok(Value::Null),
        Expr::Identifier(n,s)=>self.vars.get(n).cloned().ok_or_else(||Diagnostic::error("U4001",format!("Unknown runtime name '{n}'."),Some(*s))),
        Expr::Call{callee,args,span}=>{
            if let Some(ent)=self.entities.get(callee).cloned(){let mut fields=BTreeMap::new();for p in &ent.properties{fields.insert(p.name.clone(),Value::Null);}for(named,e) in args{let Some(n)=named else{return Err(Diagnostic::error("U4002","Entity construction requires named arguments.",Some(*span)));};let v=self.eval(e)?;fields.insert(n.clone(),v);}return Ok(Value::Entity{type_name:callee.clone(),fields});}
            if let Some(f)=self.functions.get(callee).cloned(){let old=self.vars.clone();for (idx,p) in f.params.iter().enumerate(){if let Some((_,a))=args.get(idx){let v=self.eval(a)?;self.vars.insert(p.name.clone(),v);}}let result=match self.exec_block(&f.body)?{Flow::Return(v)=>v,Flow::Continue=>Value::Null};self.vars=old;return Ok(result);}
            Err(Diagnostic::error("U4003",format!("Unknown callable '{callee}'."),Some(*span)))
        }
        Expr::Property{object,name,span}=>match self.eval(object)?{Value::Entity{fields,..}=>fields.get(name).cloned().ok_or_else(||Diagnostic::error("U4004",format!("Missing property '{name}'."),Some(*span))),other=>Err(Diagnostic::error("U4005",format!("Cannot access property on {}.",display(&other)),Some(*span)))},
        Expr::Has{subject,property,negated,..}=>{let present=match self.eval(subject)?{Value::Entity{fields,..}=>fields.get(property).map(|v|!matches!(v,Value::Null)).unwrap_or(false),_=>false};Ok(Value::Boolean(if *negated{!present}else{present}))}
        Expr::IsProperty{subject,property,negated,..}=>{let state=match self.eval(subject)?{Value::Entity{fields,..}=>matches!(fields.get(property),Some(Value::Boolean(true))),_=>false};Ok(Value::Boolean(if *negated{!state}else{state}))}
        Expr::Exists{expr,..}=>Ok(Value::Boolean(!matches!(self.eval(expr)?,Value::Null))),
        Expr::Unary{op,expr,span}=>{let v=self.eval(expr)?;match op{UnaryOp::Not=>Ok(Value::Boolean(!truthy(&v)?)),UnaryOp::Negate=>match v{Value::Number(n)=>Ok(Value::Number(-n)),_=>Err(Diagnostic::error("U4006","Unary '-' requires a number.",Some(*span)))}}}
        Expr::Binary{left,op,right,span}=>{let l=self.eval(left)?;if *op==BinaryOp::And && !truthy(&l)?{return Ok(Value::Boolean(false));}if *op==BinaryOp::Or && truthy(&l)?{return Ok(Value::Boolean(true));}let r=self.eval(right)?;self.binary(l,*op,r,*span)}
    }}
    fn binary(&self,l:Value,op:BinaryOp,r:Value,span:crate::token::Span)->Result<Value,Diagnostic>{use BinaryOp::*;match op{
        And=>Ok(Value::Boolean(truthy(&l)?&&truthy(&r)?)),Or=>Ok(Value::Boolean(truthy(&l)?||truthy(&r)?)),Eq=>Ok(Value::Boolean(l==r)),NotEq=>Ok(Value::Boolean(l!=r)),
        Greater|GreaterEq|Less|LessEq=>{let(a,b)=nums(l,r,span)?;Ok(Value::Boolean(match op{Greater=>a>b,GreaterEq=>a>=b,Less=>a<b,LessEq=>a<=b,_=>false}))},
        Add|Sub|Mul|Div=>{let(a,b)=nums(l,r,span)?;if op==Div&&b==0.0{return Err(Diagnostic::error("U4007","Division by zero.",Some(span)));}Ok(Value::Number(match op{Add=>a+b,Sub=>a-b,Mul=>a*b,Div=>a/b,_=>0.0}))}
    }}
}
fn nums(l:Value,r:Value,span:crate::token::Span)->Result<(f64,f64),Diagnostic>{if let(Value::Number(a),Value::Number(b))=(l,r){Ok((a,b))}else{Err(Diagnostic::error("U4008","Numeric operation requires numbers.",Some(span)))}}
fn truthy(v:&Value)->Result<bool,Diagnostic>{match v{Value::Boolean(b)=>Ok(*b),_=>Err(Diagnostic::error("U4009","Condition value is not Boolean.",None))}}
fn display(v:&Value)->String{match v{Value::Text(s)=>s.clone(),Value::Number(n)=>if n.fract()==0.0{format!("{n:.0}")}else{n.to_string()},Value::Boolean(b)=>b.to_string(),Value::Null=>"null".into(),Value::Entity{type_name,..}=>format!("<{type_name}>")}}
