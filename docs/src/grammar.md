This is the complete Extended Backus Normal Form (eBNF) grammar definition for Tortuga.

# Syntax Grammar
The syntactic grammar of `Tortuga` is used to parse a linear sequence of tokens into a nested syntax tree structure. The root of the grammar matches an entire `Tortuga` program (or a sequence of comparisons to make the interpreter more useful).

```ebnf
program     = expressions | comparisons EOF ;
expressions = expression+ ;
comparisons = expression comparison+ ;
comparison  = comparator expression ;
```

## Expression
A program is a series of expressions. Expressions produce values. `Tortuga` has a number of binary operators with different levels of precedence. Some grammars for languages do not directly encode the precedence relationships and specify that elsewhere. Here, we use a separate rule for each precedence level to make it explicit.

```ebnf
expression = assignment | arithmetic ;
assignment = function "=" block ;
block      = "[" expression expression+ "]" | expression ;

arithmetic = epsilon ;
epsilon    = modulo ( "~" modulo )? ;
modulo     = sum ( "%" sum )* ;
sum        = product ( ( "+" | "-") product )* ;
product    = power ( ( "*" | "/" ) power )* ;
power      = primary ( "^" primary )* ;

call       = primary arguments* ;
primary    = number | IDENTIFIER | grouping ;
number     = "-"? NUMBER ;
grouping   = "(" expression ")" ;
```

## Pattern Rules
The grammar allows pattern-matching in function definitions instead of having built-in control flow. These rules define the allowed patterns.

```ebnf
pattern    = function | refinement | bounds ;
function   = name parameters? ;
refinement = name comparator arithmetic ;
bounds     = arithmetic inequality name inequality arithmetic ;
```

## Utility Rules
To keep the above rules a little cleaner, some grammar is split out into a few reused helper rules.

```ebnf
arguments  = "(" expression ( "," expression )* ")" ;
parameters = "(" pattern ( "," pattern )* ")" ;

name       = "_" | "@" IDENTIFIER ;
inequality = "<" | "<=" | ">" | ">=" ;
equality   = "=" | "<>" ;
comparator = equality | inequality ;
```

# Lexical Grammar
The lexical grammar is used during lexical analysis to group characters into tokens. Where the syntax is [context free](https://en.wikipedia.org/wiki/Context-free_grammar), the lexical grammar is [regular](https://en.wikipedia.org/wiki/Regular_grammar) -- note that there are no recursive rules.

```ebnf
IDENTIFIER  = XID_START XID_CONTINUE* ;
NUMBER      = NONZERO DIGIT? "#" ( "0" | NATURAL | REAL | FRACTION) ;
NATURAL     = NZ_ALPHANUM ALPHANUM* ( "." "0"? )? ;
REAL        = NZ_ALPHANUM ALPHANUM* "." ALPHANUM*? NZ_ALPHANUM ;
FRACTION    = "0"? "." ALPHANUM*? NZ_ALPHANUM ;

NZ_ALPHANUM = NZ_DIGIT | ALPHA ;                
ALPHANUM    = DIGIT | ALPHA ;
ALPHA       = "a" ... "z" | "A" ... "Z" ;
NZ_DIGIT    = "1" ... "9" ;
DIGIT       = "0" ... "9" ;
```

# Operators
The associativity and precedence of the various operators in Tortuga are defined below.

| Precedence | Operation                | Symbol | Left Associative | Right Associative | Non-Associative |
|:-----------|:-------------------------|--------|------------------|-------------------|-----------------|
| 1          | Epsilon                  | ~      |                  |                   | X               |
| 2          | Modulo                   | %      | X                | X                 |                 |
| 3          | Add                      | +      | X                | X                 |                 |
| 3          | Subtract                 | -      | X                |                   |                 |
| 4          | Multiply                 | *      | X                | X                 |                 |
| 4          | Divide                   | /      | X                |                   |                 |
| 5          | Exponent                 | ^      |                  | X                 |                 |
| 6          | Pattern Match            | =      |                  | X                 |                 |
| 7          | Inequality               | <>     | X                | X                 |                 |
| 7          | Less Than                | <      | X                | X                 |                 |
| 7          | Less Than or Equal To    | <=     | X                | X                 |                 |
| 7          | Greater Than             | >      | X                | X                 |                 |
| 7          | Greater Than or Equal To | >=     | X                | X                 |                 |