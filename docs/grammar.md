This is the complete Extended Backus Normal Form (eBNF) grammar definition for Tortuga.

# Syntax Grammar
The syntactic grammar of `Tortuga` is used to parse a linear sequence of tokens into a nested syntax tree structure. The root of the grammar matches an entire `Tortuga` program (or a single entry in the interpreter).

```ebnf
program → expression* EOF ;
```

## Expression
A program is a series of expressions. Expressions produce values. `Tortuga` has a number of binary operators with different levels of precedence. Some grammars for languages do not directly encode the precedence relationships and specify that elsewhere. Here, we use a separate rule for each precedence level to make it explicit.

```ebnf
expression → epsilon ;

epsilon    → modulo ( "~" modulo )* ;
modulo     → sum ( "%" sum )* ;
sum        → product ( sign product )* ;
product    → power ( ( "*" | "/" ) power )* ;
power      → call ( "^" call )* ;

call       → IDENTIFIER ( "(" arguments ")" )? ;
primary    → number | "_" | IDENTIFIER ;
number     → ( sign? NUMBER ) ;
```

## Pattern Rules
The grammar allows pattern-matching in function definitions instead of having built-in control flow. These rules define the allowed types of patterns.

```ebnf
pattern  → function | range | identity ;
function → name ( "(" parameters ")" )? ;
range    → ( expression lesser )? name ( greater expression )? ;
identity → expression | name equality expression | expression equality name ; 
```

## Utility Rules
To keep the above rules a little cleaner, some of the grammar is split out into a few reused helper rules.

```ebnf
arguments  → expression ( "," expression )* ;
parameters → pattern ( "," pattern )* ;

name     → "_" | IDENTIFIER ;
sign     → "+" | "-" ;
equality → "=" | "<>" ;
lesser   → "<" | "<=" ;
greater  → ">" | ">=" ;
```

# Lexical Grammar
The lexical grammar is used during lexical analysis to group characters into tokens. Where the syntax is [context free](https://en.wikipedia.org/wiki/Context-free_grammar), the lexical grammar is [regular](https://en.wikipedia.org/wiki/Regular_grammar) -- note that there are no recursive rules.

```ebnf
IDENTIFIER              → XID_START XID_CONTINUE* ;
NUMBER                  → NONZERO DIGIT? "#" ( "0" | NATURAL | REAL | FRACTION) ;
NATURAL                 → NZ_ALPHANUM ALPHANUM* ( "." "0"? )? ;
REAL                    → NZ_ALPHANUM ALPHANUM* "." ALPHANUM*? NZ_ALPHANUM ;
FRACTION                → "0"? "." ALPHANUM*? NZ_ALPHANUM ;
                

NZ_ALPHANUM             → NZ_DIGIT | ALPHA ;                
ALPHANUM                → DIGIT | ALPHA ;
ALPHA                   → "a" ... "z" | "A" ... "Z" ;
NZ_DIGIT                → "1" ... "9" ;
DIGIT                   → "0" ... "9" ;
```