# AGENTS.md

# Agent instructions (Rust)

This repository implements **TradeLang**, a **deterministic DSL +
bytecode VM for financial time-series programs**.

Agents contributing to this repository **must prioritize**:

-   performance
-   modularity
-   determinism
-   memory safety
-   deep testing

Agents must treat the rules in this file as **non-negotiable**.

------------------------------------------------------------------------

# Repository-first reuse (non-negotiable)

Before implementing anything new, the agent **MUST**:

1.  **Read the repository structure first**
    -   understand crate layout
    -   understand module boundaries
    -   understand existing utilities
2.  **Search for existing implementations** before writing new code.

Examples to search for:

-   traits
-   helper modules
-   AST structures
-   bytecode instructions
-   runtime utilities
-   error types
-   testing utilities

Specifically check for existing:

-   parsing utilities
-   AST node types
-   bytecode instruction definitions
-   VM helpers
-   series buffer logic
-   builtin function helpers
-   error enums
-   test harness utilities

------------------------------------------------------------------------

## Prefer reuse over reimplementation

If an existing module already solves the problem:

-   extend it
-   reuse it
-   refactor it minimally

Do **not** introduce duplicate helpers.

Forbidden patterns:

    parse_expression_v2
    execute_vm_new
    series_buffer_alt

Agents must **avoid parallel implementations**.

------------------------------------------------------------------------

## Introducing new code

Only introduce new modules when:

-   reuse is impossible
-   the abstraction genuinely improves architecture

When introducing new code:

-   keep it minimal
-   keep it modular
-   keep naming consistent
-   document why reuse was impossible

------------------------------------------------------------------------

# Work style (non-negotiable)

## Design requirements

All new or modified code **MUST be**:

### Modular

Small modules with clear responsibilities.

### Abstracted where useful

Use traits when they improve:

-   testing
-   modularity
-   substitution

But **do not overengineer abstractions**.

### Reusable

Avoid embedding logic directly in handlers.

Compiler logic must live in:

    compiler/
    parser/
    vm/
    runtime/

### Readable

Prefer:

-   small functions
-   descriptive names
-   short rustdoc on public APIs

------------------------------------------------------------------------

# Typed structs + project tree organization (non-negotiable)

## No untyped blobs

All data structures must use **typed Rust structs/enums**.

Forbidden for domain boundaries:

    serde_json::Value
    HashMap<String, _>
    Vec<HashMap<...>>

Unless unavoidable (e.g. dynamic JSON inputs).

------------------------------------------------------------------------

## Define explicit types

Define explicit structs/enums for:

-   AST nodes
-   bytecode instructions
-   runtime values
-   VM state
-   series buffers
-   builtin arguments
-   compiler errors
-   runtime errors

------------------------------------------------------------------------

## Type boundaries are mandatory

Compiler layers must not leak representations.

Correct boundaries:

    source text
    ↓
    tokens
    ↓
    AST
    ↓
    typed AST
    ↓
    bytecode
    ↓
    VM execution

Modules must **not skip layers**.

Example violation:

    parser directly emitting bytecode

Parser must emit **AST only**.

------------------------------------------------------------------------

# Project tree hygiene

Types must live in the correct modules.

Expected layout:

    src/
      lexer/
      parser/
      ast/
      types/
      compiler/
      bytecode/
      vm/
      runtime/
      builtins/
      tests/

Do not define types in random files.

------------------------------------------------------------------------

# Change discipline

All changes must follow:

-   **smallest correct patch**
-   **no unrelated refactors**
-   **no silent behavior changes**

If behavior changes, **document it clearly**.

------------------------------------------------------------------------

# Quality gate (mandatory)

Before completing any task:

Run:

    cargo fmt
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test

All must pass.

No warnings allowed.

------------------------------------------------------------------------

# Testing rules (extremely important)

Tests are **mandatory** for non-trivial changes.

TradeLang is a **financial computation engine**.\
Bugs are unacceptable.

------------------------------------------------------------------------

## Required test types

### Unit tests

Test:

-   parsing
-   AST construction
-   type checking
-   bytecode generation
-   VM instruction execution

------------------------------------------------------------------------

### VM correctness tests

Verify:

-   stack correctness
-   instruction semantics
-   jumps
-   NA propagation
-   series indexing

------------------------------------------------------------------------

### Integration tests

Execute real TradeLang scripts against datasets.

Example:

    script: plot(sma(close, 14))
    dataset: OHLCV fixture

------------------------------------------------------------------------

### Regression tests

Every bug fix must include a regression test.

------------------------------------------------------------------------

### Golden tests

Compile and run scripts against fixed datasets.

Compare outputs with stored snapshots.

This ensures **deterministic results across versions**.

------------------------------------------------------------------------

# Performance rules (critical)

TradeLang VM is a **hot execution path**.

Agents must assume:

    millions of bars
    thousands of strategies

------------------------------------------------------------------------

## VM hot path rules

The VM execution loop must avoid:

-   heap allocations
-   trait objects
-   dynamic dispatch
-   unnecessary cloning

Prefer:

    match opcode

dispatch.

------------------------------------------------------------------------

## Memory rules

Series buffers must:

-   use ring buffers
-   reuse memory
-   avoid reallocation

------------------------------------------------------------------------

## Allocation rules

Allowed allocations:

-   compilation stage
-   program initialization

Forbidden allocations:

    execute_bar()
    vm_step()

------------------------------------------------------------------------

# Series semantics invariants

Series represent **time-indexed values**.

Access rules:

    x[0] current
    x[1] previous
    x[n] n bars ago

If insufficient history exists:

Return **NA**.

Series buffers must **never grow unbounded**.

------------------------------------------------------------------------

# Bytecode VM rules

Bytecode instructions must be:

-   deterministic
-   pure
-   predictable

All instructions must define:

-   stack effect
-   operand format
-   failure conditions

------------------------------------------------------------------------

# Builtin rules

Builtins must be:

-   deterministic
-   pure
-   side-effect free

Example builtins:

    sma
    ema
    rsi
    plot

No builtin may:

-   perform IO
-   access filesystem
-   access network
-   read system time

------------------------------------------------------------------------

# Failure loop

If any step fails:

1.  Read the full error output
2.  Identify the root cause
3.  Fix the issue
4.  Re-run tests

Repeat until green.

------------------------------------------------------------------------

# Efficiency rules (non-negotiable)

Agents must ensure:

### No memory leaks

Avoid:

    Arc<Mutex<Arc<Mutex<...>>>>

If cycles exist, use **Weak references**.

------------------------------------------------------------------------

### No RAM creep

All collections must be bounded.

Examples:

-   caches
-   buffers
-   queues

Must implement:

-   max size
-   eviction
-   or bounded channels

------------------------------------------------------------------------

### Concurrency rules

Avoid uncontrolled concurrency.

Preferred patterns:

    JoinSet
    Semaphore
    bounded worker pools

Never spawn unbounded tasks.

------------------------------------------------------------------------

# Cancellation + shutdown

Any long-running loop must support shutdown.

Use:

    CancellationToken
    select!

All spawned tasks must:

-   terminate
-   or be joined

------------------------------------------------------------------------

# Proof requirements

If code could cause memory growth:

The PR must include:

-   a cap
-   a test
-   or proof of bounded memory

------------------------------------------------------------------------

# Agent behavior expectations

Agents contributing to this repository must:

-   prioritize **determinism**
-   prioritize **performance**
-   write **tests for every feature**
-   preserve **VM invariants**
-   avoid premature abstraction
-   maintain **clear architecture boundaries**

Agents must **never merge code that compromises determinism, VM
performance, or test coverage**.