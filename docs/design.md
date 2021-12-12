# Design
The following are a set of design decisions for Tortuga roughly separated into categories.

## Goals
- Sending messages is the only language supported side-effect (i.e. statement).
- All functions are side-effect free.
- All variables are immutable.
- All data types are immutable.
- Compiler warnings are reserved exclusively for deprecated functionality to be removed in a future major version. The compiler can optionally fail when warning are found to ensure future compatibility.
- Compiler attempts to find as many errors in a single run as possible.
- Focus on concurrent performance and ease of use; single-threaded performance is not a focus.
- No key words in the grammar. Prefer mathematical symbols to C-like syntax, and C-like syntax to key words.

## Non-Goals
- Space efficiency.
- Single-threaded performance.
- Low-level (i.e., systems) programming.
- Object-Oriented Programming.

# Syntax Grammar
The syntactic grammar of `Tortuga` is used to parse a linear sequence of tokens into a nested syntax tree structure. The root of the grammar matches an entire `Tortuga` program (or a single entry in the interpreter).

```ebnf
program → block* EOF;
```

## Declarations
A `program` is a series of `declaration`s, which are the expressions that bind function declarations, perform arithmatic operations, etc.

```ebnf
functionDefinition  → IDENTIFIER "(" parameters? ")" "=" block ;

block               → declaration | "[" declaration declaration+ "]" ;
declaration         → functionDefinition | expression ;
```

## Expression
Expressions produce values. `Tortuga` has a number of binary operators with different levels of precedence. Some grammars for languages do not directly encode the precedence relationships and specify that elsewhere. Here, we use a separate rule for each precedence level to make it explicit.

```ebnf
expression → mod ;

mod -> term ( "%" term )* ;
term → factor ( sign factor )* ;
factor → exponent ( ( "*" | "/" ) exponent )* ;
exponent → primary ( "^" primary )* ;

primary → IDENTIFIER | number | "(" expression ")" | functionCall ;
functionCall → IDENTIFIER "(" arguments? ")" ;
```

## Pattern Rules
The grammar allows pattern-matching in function definitions instead of having built-in control flow.

```ebnf
pattern    → IDENTIFIER | comparison;
comparison → expression ( comparisonOperator expression )* ;
```

## Utility Rules
To keep the above rules a little cleaner, some of the grammar is split out into a few reused helper rules.

```ebnf
arguments          → expression ( "," expression )* ;
parameters         → pattern ( "," pattern )* ;

number             → sign? NUMBER | NUMBER_WITH_RADIX ;

sign               → "+" | "-" ;
comparisonOperator → "<" | ">" | "=" | "<>" | "<=" | ">=" ;
```

# Lexical Grammar
The lexical grammar is used during lexical analysis to group characters into tokens. Where the syntax is [context free](https://en.wikipedia.org/wiki/Context-free_grammar), the lexical grammar is [regular](https://en.wikipedia.org/wiki/Regular_grammar) -- note that there are no recursive rules.

```ebnf
NUMBER                  → DECIMAL ;
NUMBER_WITH_RADIX       → DIGIT+ "#" ( "+" | "-" )? BASE36 ;
IDENTIFIER              → \{alphabetic} ( ( "_" | \{alphanumeric} )*  \{alphanumeric} )? ;

BASE36                  → DIGIT36+ ( "." DIGIT36* )? | "." DIGIT36+ ;
DECIMAL                 → DIGIT+ ( "." DIGIT* )? | "." DIGIT+ ;
DIGIT36                 → ALPHA | DIGIT ;
ALPHA                   → "a" ... "z" | "A" ... "Z" ;
DIGIT                   → "0" ... "9" ;
```

### Notes
1. Numbers with a radix portion always set their sign explicitly (default to positive when no sign is present) as the sign is part of the literal. Numbers without a radix, however, always set the sign to `None` to denote that the sign is not part of the literal; the sign is a separate token altogether.

##  Data Types
- Tortuga has no String type. String programming makes internationalization more difficult. Instead the standard library will provide a mechanism to map byte string triplets (key, language, and an optional region) into a different byte string.
- No null, nil, etc. All types are actual types.
- All numbers are float-like. The goal is to provide enough accuracy where Tortuga could be used to perform calculations for science or money. Space efficiency is not a goal.
- Tortuga has no boolean type. Instead it relies on pattern matching, comparisons and dynamic dispatch to perform boolean logic.
- Numbers can be encoded in any radix up to and including 36 (e.g. 0-9, A-Z, a-z).
- Tortuga provides a fixed size binary sequence called a byte string. The string may be used as a buffer, resized, or modified via patches.

## Expressions
- Tortuga has no statements.
- Tortuga has no expression or statement terminator.
- Expressions may span more than one line.
- Order of precedence follows the mathematical order (i.e., parentheses, exponentiation, multiplication and division, addition and substraction).
- Comparisons (or variable refinements) have precedence over mathematical operations.

## Variables
- Variables must start with an alphabetic unicode character.
- Variables must not end with an underscore.
- Variables may have alphanumeric or underscore characters in the middle.
- Variables are declared implicitly when assigned or refined. Most of the trade offs mentioned in Crafting Interpreters are avoided by having immutable variables that cannot be shadowed. If a new variable with the same name is introduced in an outer scope, the existing variable assignment can be treated as a compiler error (i.e., easily detected as an error). Also, modules are a single file and are the root scope so "global" variables are limited to the current file.
- Variables may not be mutated once assigned.
- Variables cannot shadow an existing variable in an enclosing lexical scope.
- Variables may not be used in mathematical operations until the are assigned to.

## Control Flow
- Tortuga has no built-in control flow. Instead programs must rely on recursion, pattern matching and dynamic dispatch.

## Scopes
- Tortuga only has lexical scoping. Records do not have methods. Therefor every scope is static.

## Modules
- A module in Tortuga defines a set of functions, variables and procedures.
- Any of the declared items can be exported for others to consume.
- Modules cannot be nested within one another.
- Modules may be namespaced via a directory structure; directories cannot also be modules.
- Modules are not imported. Instead they are locally aliased in order to shorten their references in the local module. A function can always be referenced without an alias.
- Libraries are sets of modules that may be downloaded locally by the compiler.
- Libraries may be version using semantic versioning. Multiple versions of the same library may be referenced by a project at any given time.
- Projects define aliases for each library module that are implicitly added at the start of that project's modules. The libraries are stored locally using a directory structure to have separate namespaces for different versions. So different versions can be used even in the same project.

## Functions
- All functions in tortuga must return a value.
- Functions cannot have any side effect or mutate data or variables.
- Functions can only return a single value.
- Functions can call other functions.

## Procedures
- Procedures may call functions or send messages (the only side-effect in the language).
- Processes can be instantiated only from a procedure.

## Records
- Records are the core type in Tortuga to be used in place of classes. They are similar to Rust and Python tuples in that they are a sequence of unnamed fields. The fields are named through pattern matching.
- Tortuga provides record definitions to provide a reusable way to pattern match on a record. 

## Processes
- The corrency building block is a process.
- Processs can do anything a function can do plus: receive messages, send messages and create other processes.
- Tortuga has no synchronization or locking primitives.

## Standard Library
- Provides mathematical primitives.
- Provides concurrency building blocks (e.g. concenssus, 2-phase commit, etc.).
- Provides access to a key-value block store.
- Provides a logging process.
- Provides networking primitives (e.g. servers, clients, connections, TLS, TCP, UDP, etc.).
