# Evaluation Semantics

This page defines how PalmScript expressions and statements evaluate at runtime.

## Execution Model

PalmScript compiles a script once and evaluates it once per base-clock step.

At each step:

1. the runtime materializes the current market-series samples for the step
2. slower interval feeds advance only if their candles have fully closed by that step
3. the bytecode program executes
4. `plot`, `export`, and `trigger` outputs are collected for the step

Different market-mode feeds may construct the step inputs differently, but expression evaluation is the same once the step begins.

## Expression Categories

Expressions evaluate to the current sample of a scalar or series value.

For a series expression:

- the expression result at one step is a single current sample
- indexing addresses prior samples on that expression's own update clock

## Operator Precedence

PalmScript evaluates operators in this order, from lowest to highest precedence:

1. `or`
2. `and`
3. `==`, `!=`, `<`, `<=`, `>`, `>=`
4. `+`, `-`
5. `*`, `/`
6. unary `-`, unary `!`
7. call, indexing, and qualification postfixes

Operators of the same precedence associate left-to-right.

## Arithmetic

Arithmetic operators are `+`, `-`, `*`, and `/`.

Rules:

- both operands must be numeric, numeric series, or `na`
- if either operand is `na`, the result is `na`
- if either operand is `series<float>`, the result is `series<float>`
- otherwise the result is `float`

## Comparisons

Comparison operators are `==`, `!=`, `<`, `<=`, `>`, and `>=`.

Rules:

- `<`, `<=`, `>`, and `>=` require numeric operands
- `==` and `!=` are defined for any non-`na` operands
- mixed-type equality compares unequal
- if either operand is `na`, the result is `na`
- if either operand is a series, the result is a series boolean
- otherwise the result is `bool`

## Unary Operators

PalmScript supports:

- unary `-` for numeric operands
- unary `!` for boolean operands

Rules:

- unary operators propagate `na`
- unary `-` over `series<float>` yields `series<float>`
- unary `!` over `series<bool>` yields `series<bool>`

## Logical Operators

`and` and `or` require `bool`, `series<bool>`, or `na`.

They use deterministic three-valued logic:

### `and`

| Left | Right | Result |
| --- | --- | --- |
| `false` | `false` | `false` |
| `false` | `true` | `false` |
| `false` | `na` | `false` |
| `true` | `false` | `false` |
| `true` | `true` | `true` |
| `true` | `na` | `na` |
| `na` | `false` | `false` |
| `na` | `true` | `na` |
| `na` | `na` | `na` |

### `or`

| Left | Right | Result |
| --- | --- | --- |
| `true` | `true` | `true` |
| `true` | `false` | `true` |
| `true` | `na` | `true` |
| `false` | `true` | `true` |
| `false` | `false` | `false` |
| `false` | `na` | `na` |
| `na` | `true` | `true` |
| `na` | `false` | `na` |
| `na` | `na` | `na` |

PalmScript evaluates both operands before applying the logical operator. The language does not guarantee short-circuit evaluation, so logical expressions are analyzed and executed eagerly within the language's normal rules.

## `if` Semantics

`if` is a statement form, not an expression form.

Rules:

- the condition must evaluate to `bool`, `series<bool>`, or `na`
- `na` in an `if` condition is treated as false for branch selection
- exactly one branch executes on each step
- both branches must be syntactically present because `else` is mandatory

## Function Evaluation

User-defined functions are expression-bodied and are compiled through specialization rather than runtime dynamic dispatch.

Rules:

- argument count must match the declared parameter count
- functions are specialized by argument type and update clock
- recursive and cyclic function graphs are rejected at compile time
- a function body cannot call `plot`

## No-Lookahead Rule

PalmScript must not expose partially formed higher-interval candles.

Consequences:

- a higher-interval series changes only after that candle fully closes
- indexing on higher-interval series walks the history of fully closed higher-interval samples
- source-aware supplemental intervals follow the same rule

## Builtin Helper Semantics

Builtin helper formulas, window rules, and `na` behavior are defined in [Builtins](builtins.md).

Rules:

- helper builtins follow the update clocks of their series inputs
- helper outputs participate in `if`, indexing, and further builtin calls through the same value and `na` rules defined on this page
- `if` still treats `na` as false for branch selection, even when the condition comes from a helper such as `crossover(...)`

## Determinism

Expression evaluation is deterministic.

During strategy execution, the language semantics depend only on:

- the compiled program
- the prepared input feeds
- the configured VM limits

They do not depend on wall-clock time, filesystem access, randomness, or network access.
