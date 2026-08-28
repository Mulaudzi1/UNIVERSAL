use std::{env, fs, process};
fn main(){
    let args:Vec<String>=env::args().collect();
    if args.len()<2{usage();return;}
    match args[1].as_str(){
        "run"=>with_file(&args,|src|match universal_compiler::run(src){Ok(out)=>{for l in out.stdout{println!("{l}");}for v in out.validations{eprintln!("validation: {v}");}for a in out.actions{println!("action: {a}");}0},Err(e)=>{print_diags(&e,src);1}}),
        "check"=>with_file(&args,|src|match universal_compiler::check(src){Ok(_)=>{println!("OK");0},Err(e)=>{print_diags(&e,src);1}}),
        "build"=>with_file(&args,|src|match universal_compiler::check(src){Ok(_)=>{println!("Build check passed. V0.1 interpreter backend emits no binary yet.");0},Err(e)=>{print_diags(&e,src);1}}),
        "format"=>{eprintln!("formatter is reserved for V0.2");},
        "test"=>{println!("Run: cargo test --workspace");},
        "repl"=>{eprintln!("REPL is reserved for V0.2");},
        _=>{usage();}
    };
}
fn with_file<F:FnOnce(&str)->i32>(args:&[String],f:F){if args.len()<3{eprintln!("missing source file");process::exit(2);}let src=match fs::read_to_string(&args[2]){Ok(s)=>s,Err(e)=>{eprintln!("{}: {e}",args[2]);process::exit(2)}};process::exit(f(&src));}
fn print_diags(ds:&[universal_compiler::diagnostic::Diagnostic],src:&str){for d in ds{if let Some(s)=d.span{eprintln!("{}:{}: {} {}",s.line,s.column,d.code,d.message);if let Some(line)=src.lines().nth(s.line.saturating_sub(1)){eprintln!("  {:>4} | {}",s.line,line);let pad=" ".repeat(s.column.saturating_sub(1));eprintln!("       | {}^",pad);}}else{eprintln!("{} {}",d.code,d.message);}if let Some(h)=&d.help{eprintln!("  help: {h}");}}}
fn usage(){println!("UNIVERSAL 0.1\nusage: universal <run|check|build|format|test|repl> [file.univ]");}
