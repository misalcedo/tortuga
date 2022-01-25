This is the complete definition for supported patterns in Tortuga binding expressions.

# Patterns
The `Tortuga` Programming Language relies on pattern matching instead of control flow. Patterns apply to each of the base types.

## Tuples
* First & Rest: `{ First | Rest }`
* Named: `{ X, Y, Z }`

## Intervals
* Named: `]X, Y[`, `(X, Y]`, `(X, Y)`, etc.

## Numbers
* Parts: `X.Y`
* Natural: `X.0`
* Fractional: `0.X`

## Function
* Named: `f(x, y)`
* Pattern: `f({x, y}, [a, b])`
* Refinement: `f(x.0)`

## Refinements
## Functions
* Arity: `|X|`

## Tuples
* Cardinality: `|X|`

### Numbers
* Bounds: `x < 0`, `x > 1`, `x = (0, 100]`