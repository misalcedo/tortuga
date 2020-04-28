= Tortuga

== Background
An actor is a computational entity that, in response to a message it receives, can concurrently:

1. send a finite number of messages to other actors;
1. create a finite number of new actors;
1. designate the behavior to be used for the next message it receives.

== Grammar
```bnf
<behavior> ::= <action> | <action> <behavior>
<action> ::= <send-message> | <create-actor> | <designate-behavior>

<send-message> ::= <opt-whitespace> "(" <opt-whitespace> <reference> <opt-whitespace> <message> ")" <line-end>

<reference> ::= <letter> | <letter> <reference>

<message> ::= "" | <message-part> | <message-part> <message>
<message-part> ::= <natural-number> | <real-number>

<natural-number> ::= <digit> | <non-zero-digit> <digits>
<real-number> ::= <natural-numer> "." <digits>

<opt-whitespace> ::= " " <opt-whitespace> | ""
<line-end> ::= <opt-whitespace> <EOL> | <line-end> <line-end>

<digits> ::= <digit> | <digit> <digits>
<non-zero-digit> ::= "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
<digit> ::= "0" | <non-zero-digit>

<letter> ::= "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z" | "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z"
```