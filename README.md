# IDC: Intent-Driven Compiler

"It doesn’t crash. It reflects."  
Designed by DotDotDotPy, For those who don’t need syntax to speak logic.

## What Is IDC

IDC is not a programming language.  
It is a thought execution engine built to run logic the way you meant it, not the way the parser wished you did.

You don’t code. You declare.  
You don’t debug. You ask.

```idc
why: did it skip "respond Welcome"
```

And it tells you.

No tracebacks.  
No cryptic errors.  
Just a response like:

"respond Welcome" was skipped because stop triggered inside if invalid.

That is not a stack trace. That is an answer.

## How It Works

IDC is a runtime and interpreter written from scratch in Rust, engineered to:

- Parse declarative instructions using sentence-first indentation-aware structure
- Build structured abstract syntax trees without traditional syntax rules
- Execute logic with arithmetic, conditionals, loops, and scoped memory
- Reflect on execution in real time to explain behavior
- Support placeholder substitution, runtime memory, and event ticks

This is not syntax.  
This is structure with intention.

## Example

```idc
on user login
  verify token
  if invalid
    respond "Unauthorized"
    stop
  respond "Welcome"
```

Then:

```idc
why: did it skip "Welcome"
```

It does not break. It explains.

## Built With AI Assisted Coding. Driven By Intuition.

IDC was architected by describing the way logic should feel to write.  
AI handled the low-level generation. DotDotDotPy made it honest.

No wasted code.  
No brittle rules.  
Just behavior that maps to meaning.

"Only do what I meant."  
That was the bar.  
It passed.

## Vision

IDC is the first introspective runtime designed for post-syntax logic.

Imagine a language that:

- Never panics, just reflects
- Tells you what it skipped and why
- Lets you debug with natural questions
- Adapts to thought, not tradition

You don’t write programs. You write what happens.

## Project Status

| Phase     | Description                                  | Status       |
|-----------|----------------------------------------------|--------------|
| Phase 1   | Grammar, Tokenizer, AST                      | Complete     |
| Phase 2   | Interpreter Core, Flow Logic, Arithmetic     | Complete     |
| Phase 3   | Runtime Engine, Scoped Memory, Tick Flow     | In Progress  |
| Phase 4   | Introspection, Query Support, Trace Logs     | Coming       |
| Phase 5   | Extensible Plugins, Open Token System        | Planned      |

## Sample Snippet

```idc
on start
  load a 3
  load b 4
  load result "{{a}} + {{b}}"
  respond_event "{{result}} + 1"

  if "{{result}} > 5"
    respond "Above threshold"
  unless "{{result}} > 10"
    respond "Still under limit"
```

Declarative memory. Evaluated math. Conditional logic. Dynamic introspection.  
No compiler complaints. No boilerplate. Just flow.

## Core Principles

- Instruction over syntax  
- Fallbacks over crashes  
- Clarity over verbosity  
- Scoped logic  
- Reflexive memory  
- Runtime explainability

This is AI-readable code.  
No scaffolding. Just intention.

## Who Made This

Designed by: DotDotDotPy 
Co-engineered with AI  
Maintained by: DotDotDotPy, unless he gets funded, in which case someone else is pushing through the pain

## Real Examples

Every feature lives in the examples directory.  
All real. All tested.  
No synthetic benchmarks. No boilerplate clones. Just expressive logic.

## License

MIT. Do what you want.  
Just don’t claim you wrote it if you didn’t have to pull apart your brain to make it feel right.

## Final Thought

You’re not here to prove you can code.  
You’re here to prove you can think.

IDC runs thoughts.  
Clean. Sharp. Explainable.

Welcome to logic that flows like language.

Now write what you meant.
