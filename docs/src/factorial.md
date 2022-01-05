# Factorial

In this chapter we will implement [factorial](https://en.wikipedia.org/wiki/Factorial) using the Tortuga Programming Language. In mathematics, the `factorial` of a non-negative integer `n`, denoted by `n!`, is the product of all positive integers less than or equal to `n`. The factorial of `n` also equals the product of `n` with the next smaller factorial. The factorial of `0` is equal to `1` (i.e., `0! = 1`).

## Implementation
 All numbers in the Tortuga Programming Language are signed real numbers. Since factorial is only defined for non-negative integers, our implementation will round all real numbers to their nearest integer and report an error for non-negative numbers.

## Copy & Paste
Create a file named `factorial.ta` with the following contents:

```tortuga
@floor(@n >= 0) = n - (n % 1)

@round(@n >= 0) = round(floor(n), n % 1)
@round(@n >= 0, @remainder >= 0.5) = 1 + n
@round(@n >= 0, @remainder < 0.5) = n

@factorial(@n = 0) = 1
@factorial(@n > 0) = [
    @i = round(n)
    i * factorial(i - 1)
]

factorial(9)
```

## Run
To run the file use the command-line interface from your favorite terminal:

```console
tortuga run factorial.ta
```

You should see the value `362880` printed to your terminal.
The Tortuga Programming Language automatically prints the value of the last expression in a program.