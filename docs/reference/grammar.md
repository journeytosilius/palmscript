# Grammar

This page is the normative grammar for PalmScript as implemented in this repository.

The productions below define the accepted parser surface. Rules that depend on name resolution, interval ordering, or typing are defined in later reference chapters.

## Program

```text
program                ::= separator* item* EOF
item                   ::= interval_decl
                         | source_decl
                         | use_decl
                         | function_decl
                         | stmt

interval_decl          ::= "interval" interval
source_decl            ::= "source" ident "=" source_template "(" string_literal ")"
source_template        ::= ident "." ident
use_decl               ::= "use" interval
                         | "use" ident interval
function_decl          ::= "fn" ident "(" param_list? ")" "=" expr
param_list             ::= ident ("," ident)*
```

## Statements

```text
stmt                   ::= let_stmt
                         | export_stmt
                         | trigger_stmt
                         | if_stmt
                         | expr_stmt

let_stmt               ::= "let" ident "=" expr
export_stmt            ::= "export" ident "=" expr
trigger_stmt           ::= "trigger" ident "=" expr
if_stmt                ::= "if" expr block "else" else_tail
else_tail              ::= if_stmt
                         | block
expr_stmt              ::= expr
block                  ::= "{" separator* stmt* "}"
```

## Expressions

```text
expr                   ::= or_expr
or_expr                ::= and_expr ("or" and_expr)*
and_expr               ::= cmp_expr ("and" cmp_expr)*
cmp_expr               ::= add_expr (cmp_op add_expr)*
cmp_op                 ::= "==" | "!=" | "<" | "<=" | ">" | ">="
add_expr               ::= mul_expr (("+" | "-") mul_expr)*
mul_expr               ::= unary_expr (("*" | "/") unary_expr)*
unary_expr             ::= ("-" | "!") unary_expr
                         | postfix_expr
postfix_expr           ::= primary_expr postfix*
postfix                ::= call_suffix
                         | index_suffix
                         | source_suffix
call_suffix            ::= "(" arg_list? ")"
index_suffix           ::= "[" expr "]"
source_suffix          ::= "." ident
                         | "." interval "." ident
arg_list               ::= expr ("," expr)*
```

## Primary Expressions

```text
primary_expr           ::= number
                         | "true"
                         | "false"
                         | "na"
                         | string_literal
                         | ident
                         | interval "." market_field
                         | "(" expr ")"
```

## Lexical Nonterminals

```text
market_field           ::= "open" | "high" | "low" | "close" | "volume" | "time"
interval               ::= one of the literals listed in [Interval Table](intervals.md)
ident                  ::= identifier token
string_literal         ::= string token
number                 ::= numeric literal token
separator              ::= newline | ";"
```

## Binding And Precedence

PalmScript parses binary operators with the following precedence, from lowest to highest:

1. `or`
2. `and`
3. `==`, `!=`, `<`, `<=`, `>`, `>=`
4. `+`, `-`
5. `*`, `/`
6. unary `-`, unary `!`
7. call `(...)`, indexing `[...]`, and source/field qualification with `.`

Operators within one precedence level associate left-to-right.

## Required Semantic Restrictions

The grammar does not by itself make a program valid. The implementation additionally requires:

- a script must declare exactly one base `interval`
- `interval`, `source`, `use`, `fn`, `export`, and `trigger` must appear only at the top level
- every `if` must have an `else`
- string literals are accepted lexically but are semantically valid only inside `source` declarations
- only identifiers may be called
- series indexing must use a non-negative integer literal
- source, interval, scope, and type rules are enforced as described in the other `Reference` pages
