This is the complete Extended Backus Normal Form (eBNF) grammar definition for Tortuga.

# Syntax Grammar
The syntactic grammar of `Tortuga` is used to parse a linear sequence of tokens into a nested syntax tree structure. The root of the grammar matches an entire `Tortuga` program (or a sequence of comparisons to make the interpreter more useful).

```ebnf
program → expression+ EOF ;
program → expression ( comparison expression )+ EOF ;
```

## Expression
A program is a series of expressions. Expressions produce values. `Tortuga` has a number of binary operators with different levels of precedence. Some grammars for languages do not directly encode the precedence relationships and specify that elsewhere. Here, we use a separate rule for each precedence level to make it explicit.

```ebnf
expression → epsilon | assignment ;
assignment → "@" function "=" block ;
block      → expression | "[" expression expression+ "]" ;

epsilon    → modulo ( "~" modulo )* ;
modulo     → sum ( "%" sum )* ;
sum        → product ( sign product )* ;
product    → power ( ( "*" | "/" ) power )* ;
power      → primary ( "^" primary )* ;

primary    → number | call | grouping ;
number     → sign? NUMBER ;
call       → IDENTIFIER ( "(" arguments ")" )* ;
grouping   → "(" expression ")" ;
```

## Pattern Rules
The grammar allows pattern-matching in function definitions instead of having built-in control flow. These rules define the allowed patterns.

```ebnf
pattern  → function | range | identity ;
function → name ( "(" parameters ")" )? ;
range    → number inequality name | ( number inequality )? name inequality number ;
identity → number | name equality number | number equality name ;
```

## Utility Rules
To keep the above rules a little cleaner, some grammar is split out into a few reused helper rules.

```ebnf
arguments   → expression ( "," expression )* ;
parameters  → pattern ( "," pattern )* ;

name        → "_" | IDENTIFIER ;
sign        → "+" | "-" ;
inequality  → "<" | "<=" | ">" | ">=" ;
equality    → "=" | "<>" ;
comparison  → equality | inequality ;
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