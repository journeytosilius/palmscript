# Learn PalmScript

PalmScript public documentation is organized around:

- the language for writing strategies
- examples that show how scripts are written and used

## What You Do With PalmScript

Typical workflow:

1. write a `.ps` script
2. declare a base `interval`
3. declare one or more `source` bindings
4. validate it in the browser IDE
5. run it over historical data in the app

## Long Optimize Runs

For long CLI tuning jobs:

- use `palmscript run optimize ...` when you want a foreground result immediately
- use `palmscript runs submit optimize ...` when the search should continue in local durable state and keep every completed candidate
- come back later with `palmscript runs status <run-id>`, `palmscript runs show <run-id>`, `palmscript runs tail <run-id>`, or `palmscript runs best <run-id> --preset-out best.json`

## What To Read Next

- First runnable flow: [Quickstart](quickstart.md)
- First complete strategy walkthrough: [First Strategy](first-strategy.md)
- Big-picture language tour: [Language Overview](language-overview.md)
- Exact rules and semantics: [Reference Overview](../reference/overview.md)

## Documentation Roles

- `Learn` explains how to use PalmScript effectively.
- `Reference` defines what PalmScript means.
