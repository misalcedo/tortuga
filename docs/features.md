# Features
Stand-out features of Tortuga:
- Concurrency Built-in
- Process Supervision
- Pattern Matching
- Bit & Byte Strings
- Internationalization (Text Reference + Locale)
- Accurate arithmetic with real numbers
- Constraint programming

# Tenets

- Sending messages is the only language supported side-effect (i.e. statement).
- All functions are pure (side-effect free).
- All variables are immutable.
- Compiler warnings are reserved exclusively for deprecated functionality to be removed in a future major version. The compiler can optionally fail when warning are found to ensure future compatibility.
- Compiler attempts to find as many errors in a single run as possible.
- Focus on concurrent performance and ease of use; single-threaded performance is not a focus.

# Open questions
- How to support warnings and errors?
- How to make the scanner stage infallible, but still pass invalid tokens to the parser?