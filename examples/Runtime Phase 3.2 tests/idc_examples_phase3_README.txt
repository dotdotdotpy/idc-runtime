IDC Examples Folder - Interpreter Test Suite (Phase 3)
======================================================

This folder contains updated .idc script files used to validate the functionality of the IDC interpreter and runtime during Phase 3. Each script is designed to stress test specific engine components and demonstrate scoped memory, event chaining, tick-based logic, and conditional reasoning.

────────────────────────────────────────────
Phase 3 Feature & Runtime Test Scripts
────────────────────────────────────────────

1. a.idc
Purpose: Test full runtime event flow and periodic triggers.

Script Overview:
- Registers on start and end
- Executes respond_event and wait
- Uses `every` for timed event simulation
- Demonstrates memory and fallback output

────────────────────────────────────────────

2. b.idc / c.idc
Purpose: Basic loop and memory progression with `respond_event`.

Script Overview:
- Loads `counter`, increments inside an `every` loop
- Calls respond_event with dynamic arithmetic
- Responds final memory after loop

────────────────────────────────────────────

3. d.idc
Purpose: Math logic with priority and memory interaction.

Script Overview:
- Loads values a and b
- Calculates `result` with math precedence
- Uses respond_event for dynamic math output

────────────────────────────────────────────

4. e.idc
Purpose: Parenthetical math expression test.

Script Overview:
- Loads a single value `x`
- Calculates and responds to a math expression with parentheses

────────────────────────────────────────────

5. f.idc
Purpose: Conditional testing and math validation.

Script Overview:
- Uses arithmetic and conditionals in one script
- Responds based on evaluated expressions using `if` and `unless`
- Tests complex memory interactions

────────────────────────────────────────────

6. g.idc
Purpose: Event triggering and conditional math flow.

Script Overview:
- Triggers math_event from start block
- Computes a math expression and uses `if`/`unless` logic to branch

────────────────────────────────────────────

7. trigger_chain.idc
Purpose: Deep event chaining and scoped memory testing.

Script Overview:
- Triggers a → b → c event chain
- Loads and mutates memory at each level
- Confirms nested memory resolution

────────────────────────────────────────────

8. invalid_trigger.idc
Purpose: Fallback test for missing or unknown events.

Script Overview:
- Attempts to trigger a non-existent event
- Handles fallback via `on missing`

────────────────────────────────────────────

9. recursive_trigger.idc
Purpose: Test for reentrancy protection.

Script Overview:
- Triggers loop_a recursively
- Used to test future reentrancy guards

────────────────────────────────────────────

10. ticks.idc
Purpose: Tick-driven runtime simulation.

Script Overview:
- Simulates periodic logic using `every` X tick blocks
- Increments cycles and responds to tick-based logic

────────────────────────────────────────────
Usage
────────────────────────────────────────────

Run from CLI using:

    cargo run -- --run examples/<filename>.idc

Other debug options:

    cargo run -- --tokens examples/<file>.idc
    cargo run -- --ast examples/<file>.idc
    cargo run -- --semantics examples/<file>.idc

────────────────────────────────────────────
Stubbed Token Notes
────────────────────────────────────────────

The following tokens are parsed and recognized but not yet implemented. They are handled gracefully and included in the fallback flow.

    remember, get, capture, define, alias,
    print, log, show, alert, stop, retry, then,
    authorize, purge, reset, connect, disconnect,
    save, update, check, verify, use, from, to,
    with, as, every, route

────────────────────────────────────────────