# UNIVERSAL Language Specification — Draft 0.1

## Lexical rules

Identifiers begin with ASCII letter or `_`, followed by ASCII letters, digits, or `_`. Keywords are case-insensitive in V0.1. String literals use double quotes. `#` and `//` begin line comments.

## Blocks

Blocks are terminated by `END`. Newlines delimit statements. Indentation is formatting only.

## EBNF

```ebnf
program         = { newline | statement } , EOF ;
statement       = entity-decl | function-decl | when-stmt | variable-decl
                | print-stmt | validate-stmt | return-stmt | call-stmt | action-stmt ;

entity-decl     = "ENTITY" , identifier , newline , { property-decl , newline } , "END" ;
property-decl   = identifier , ":" , type-ref , [ "?" ] ;
type-ref        = identifier ;

function-decl   = "FUNCTION" , identifier , "(" , [ parameter-list ] , ")"
                , [ "->" , type-ref ] , newline , block , "END" ;
parameter-list  = parameter , { "," , parameter } ;
parameter       = identifier , [ ":" , type-ref ] ;

variable-decl   = identifier , "=" , expression , newline ;
when-stmt       = "WHEN" , expression , newline , block
                , { "ELSE" , "WHEN" , expression , newline , block }
                , [ "OTHERWISE" , newline , block ] , "END" ;

print-stmt      = "PRINT" , expression ;
validate-stmt   = "VALIDATE" , expression ;
return-stmt     = "RETURN" , expression ;
action-stmt     = identifier , { identifier | string-literal } , newline ;

expression      = or-expr ;
or-expr         = and-expr , { "OR" , and-expr } ;
and-expr        = comparison , { "AND" , comparison } ;
comparison      = additive , { ( "==" | "!=" | ">" | ">=" | "<" | "<=" ) , additive } ;
additive        = multiplicative , { ( "+" | "-" ) , multiplicative } ;
multiplicative  = unary , { ( "*" | "/" ) , unary } ;
unary           = [ "NOT" | "-" ] , postfix ;
postfix         = primary , { property-access | exists-test | has-test | is-test } ;
property-access = "." , identifier ;
exists-test     = "EXISTS" ;
has-test        = "HAS" , [ "a" | "an" ] , identifier
                | "DOES" , "NOT" , "HAVE" , [ "a" | "an" ] , identifier ;
is-test         = "IS" , [ "NOT" ] , identifier ;
primary         = string-literal | number-literal | "true" | "false" | "null"
                | identifier | call | "(" , expression , ")" ;
call            = identifier , "(" , [ argument-list ] , ")" ;
argument-list   = argument , { "," , argument } ;
argument        = [ identifier , ":" ] , expression ;
```

## Readable conditions

`employee has a scorecard` checks whether the declared `scorecard` property exists and has a non-null runtime value. `employee is active` is shorthand for a Boolean entity property named `active`. Neither production permits synonyms chosen at runtime.

## Optional values

`Type?` means optional. Missing constructor fields are initialized to `null` in V0.1. Accessing a declared optional property is valid, but programs should use `EXISTS`/`HAS` before relying on a value. Full flow-sensitive optional narrowing is scheduled for a later type-system phase.
