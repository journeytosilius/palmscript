# Types and Values

PalmScript operates over scalar numbers, scalar booleans, series of those values, `na`, and `void`.

## Concrete Types

The implementation distinguishes these concrete types:

- `float`
- `bool`
- `series<float>`
- `series<bool>`
- `void`

`void` is the result type of expressions such as `plot(...)` that do not yield a reusable value.

## Primitive Values

PalmScript values have the following runtime forms:

- numeric values are `f64`
- boolean values are `true` or `false`
- `na` is the missing-value sentinel
- `void` is not a user-writable literal

## Series Types

Series values are time-indexed streams.

A series type:

- advances on an update clock
- retains bounded history
- exposes its current sample when used in expressions
- may yield `na` at a given sample

Market fields are series values. Indicator builtins also return series values.

## `na`

`na` is part of normal language semantics. It is not a runtime exception.

`na` may arise from:

- insufficient history for indexing
- indicator warm-up
- missing data on a source-aware base-clock step
- arithmetic or comparisons where an operand is already `na`
- explicit use of the `na` literal

## Series And Scalar Combination

PalmScript allows scalar/series mixing in expressions when the underlying operator accepts the operand categories.

Rules:

- if either accepted operand is `series<float>`, arithmetic produces `series<float>`
- if either accepted operand is `series<bool>`, logical operations produce `series<bool>`
- if either accepted operand is `series<float>`, numeric comparisons produce `series<bool>`
- equality over any series operand produces `series<bool>`

This is value lifting, not implicit materialization of an unbounded series. Evaluation still follows the update clocks described in [Evaluation Semantics](evaluation-semantics.md).

## `na` In Type Checking

`na` is accepted anywhere a numeric or boolean expression may later be required, subject to the surrounding construct.

Examples:

- `plot(na)` is valid
- `export x = na` is valid
- `trigger t = na` is valid
- `if na { ... } else { ... }` is valid

## Boolean Logic

`and` and `or` use PalmScript's three-valued logic.

They do not coerce `na` to `false`. Their runtime truth table is defined in [Evaluation Semantics](evaluation-semantics.md).

## Output Normalization

Output declarations normalize their value types as follows:

- `export` over numeric, series numeric, or `na` yields `series<float>`
- `export` over bool or series bool yields `series<bool>`
- `trigger` always yields `series<bool>`

See [Outputs](outputs.md) for the exact output behavior.
