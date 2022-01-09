# Design
The following are a set of design decisions for Tortuga roughly separated into categories.

## Goals
- Sending messages is the only language supported side-effect (i.e. statement).
- All functions are side-effect free.
- Variables are constant functions with no arguments.
- Functions may only be declared once.
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

##  Data Types
- Tortuga has no String type. String programming makes internationalization more difficult. Instead the standard library will provide a mechanism to map byte string triplets (key, language, and an optional region) into a different byte string.
- No null, nil, etc. All types are concrete types.
- All numbers are float-like. The goal is to provide enough accuracy where Tortuga could be used to perform calculations for science or money. Space efficiency and speed are not goals for numerical operations, only accuracy and precision.
- Tortuga has no boolean type and no control flow. Instead it relies on pattern matching, comparisons and dynamic dispatch to perform control flow and on regular numbers for boolean logic.
- Numbers can be encoded in any radix up to and including 36 (e.g. 0-9, A-Z, a-z).
- Tortuga provides a fixed size binary sequence called a byte string. The string may be used as a buffer, resized, or modified via patches. Byte strings have an optional padding length to denot how much of the final byte is padding for bit sequences rather than byte sequences.
- Tortuga has a tuple type that can hold arbitrary types in each field. Tuple are callable with an index to extract a field. Also, tuples can be used as linked-lists (i.e. first and rest).
- Tortuga has a range type to that supports inclusive and exclusive bounds at the low and high end.

## Expressions
- Tortuga has no statements. Every operation returns a value. For example sending a message returns the sent message to allow sending the same message to multiple recipients.
- Tortuga has no expression or statement terminator (i.e., no semicolon at the end of a statement).
- Expressions may span more than one line at any point.
- Order of precedence follows the mathematical order (i.e., parentheses, exponentiation, multiplication and division, addition and subtraction).
- Comparisons and pattern matching have a lower precedence than mathematical operations.

## Identifiers
- Identifiers must start with a unicode character withe the `XID_START` property, followed by zero or more `XID_CONTINUE` characters.
- Variables are declared implicitly when in a pattern match. Most of the trade offs mentioned in Crafting Interpreters are avoided by having immutable variables that cannot be shadowed. If a new variable with the same name is introduced in an outer scope, the existing variable assignment can be treated as a compiler error (i.e., easily detected as an error). Also, modules are a single file and are the root scope so "global" variables are limited to the current file.
- Variables may not be mutated once assigned.
- Variables cannot shadow an existing variable in an enclosing lexical scope.
- Variables may not be used in mathematical operations until assigned to.

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
- The building block for concurrency is a process.
- Processes can do anything a function can do plus: receive messages, send messages and create other processes.
- Tortuga has no synchronization or locking primitives.

## Standard Library
- Provides mathematical primitives.
- Provides concurrency building blocks (e.g. consensus, 2-phase commit, etc.).
- Provides access to a key-value block store.
- Provides a logging process.
- Provides networking primitives (e.g. servers, clients, connections, TLS, TCP, UDP, etc.).
