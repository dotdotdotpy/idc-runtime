IDC Examples Folder - Interpreter Test Suite
============================================

This folder contains .idc script files used to validate functionality and stress-test various components of the IDC interpreter. These scripts are grouped into thematic categories and provide full coverage of:

- Flow control (if, else, while, break, continue)
- Arithmetic evaluation with dynamic memory substitution
- Memory operations (load, delete, respond)
- Semantic robustness (graceful fallback on unknown tokens)
- Full 40-token recognition (including stubbed commands)

────────────────────────────────────────────
Core Interpreter Feature Tests
────────────────────────────────────────────

1. test_all_features.idc
Purpose: This comprehensive script validates the full functionality of the IDC interpreter by testing every major capability.

Script Overview:
- Arithmetic Logic: Loads values into `a` and `b`, then computes `sum` using `{{a}} + {{b}}`. Responds with the computed value.
- Conditional Check: If `sum` equals 5, responds with a success message. Else block is skipped correctly.
- Loop Execution: Uses a `while` loop with a counter to ensure arithmetic and memory mutation in loops function correctly.
- Memory Cleanup: Deletes `sum` and `counter`, leaving only `a` and `b` in memory.
- Token Coverage: Invokes every known stubbed token in the language to confirm 40-token coverage and correct fallback behavior.

Process:
This was built as a top-down integration test for interpreter stability. It confirms flow control, memory management, dynamic substitution, and fallback on unimplemented nodes.

────────────────────────────────────────────

2. test_arithmetic.idc
Purpose: Confirms correct math parsing and evaluation for basic expressions.

Script Overview:
- Loads `a` and `b` with numeric strings.
- Computes `result` using `{{a}} + {{b}}`.
- Uses an `if` statement to check if `result == 10`, then responds based on the result.

Process:
Designed to test numeric resolution, inline math computation, and condition branching on calculated values.

────────────────────────────────────────────

3. test_token_coverage.idc
Purpose: Ensures all 40 foundational IDC tokens are recognized and handled (stubbed or real).

Script Overview:
- Executes each command in the language exactly once.
- Verifies interpreter recognizes and logs each unimplemented token gracefully.

Process:
Serves as a snapshot of interpreter readiness for future token expansions. Used to validate token mapping completeness.

────────────────────────────────────────────

4. maths_kitchen.idc
Purpose: Stress-tests the interpreter's arithmetic logic and error handling.

Script Overview:
- Arithmetic: Confirms order-of-operations using `+`, `*`, and `/`.
- Division: Tests valid division and divide-by-zero fallback.
- Error Handling: Introduces malformed expressions and invalid characters (e.g., `&`, `1 + (2 *`) to verify graceful fallback.

Process:
Validates that invalid math does not crash the runtime, and all errors are logged meaningfully with fallback output.

────────────────────────────────────────────

5. math_deluxe.idc
Purpose: Provides a clean math example with expected output for quick sanity checks.

Script Overview:
- Loads two numbers, calculates an expression with precedence, and responds with result.

Process:
Used during development to validate fast math evaluation independently of conditional logic.

────────────────────────────────────────────

6. level10_stress_test.idc
Purpose: Minimal script for confirming interpreter boot sequence.

Script Overview:
- Loads a single variable `x` from `"5"`.

Process:
Used as a lightweight smoke test during early stages of interpreter boot, file loading, and CLI parsing.

────────────────────────────────────────────
Legacy Flow & Logic Tests
────────────────────────────────────────────

7. hello_grant.idc
Purpose: Demonstrates memory initialization, conditional looping, and unknown node recovery.

Script Overview:
- Loads a counter, loops while it’s active, and adjusts its value each iteration.
- Tests conditional branching inside loop and responds differently based on condition result.
- Uses unknown node types to confirm interpreter continues executing children.
- Performs memory deletion and final memory access test.

Process:
Created to validate unknown token fallback, loop structure, memory mutation, and chained execution flow.

────────────────────────────────────────────

8. depth_limiter.idc
Purpose: Tests conditional loop termination via memory mutation.

Script Overview:
- Loads `depth`, enters loop.
- Deletes and reassigns `depth` to control loop exit.
- Confirms loop terminates when memory reaches `0`.

Process:
Created to verify safe conditional exit and reassignment within loops.

────────────────────────────────────────────

9. unknown_but_valid.idc
Purpose: Validates interpreter continuation through unknown node types.

Script Overview:
- Includes several unknown nodes with children to verify interpreter still runs contained logic.
- Checks `if`/`else` logic and memory mutation across unrecognized branches.

Process:
Built to confirm robust fallback for malformed or placeholder tokens during execution.

────────────────────────────────────────────

10. depth_loop_test.idc
Purpose: Exercises nested loops and cross-loop memory transitions.

Script Overview:
- Outer loop uses `depth` variable, inner loop uses `level`.
- Each loop performs different memory operations and uses `continue` and `break` to modify control flow.

Process:
Tests interpreter’s loop stack system and verifies correct control state persistence between loop layers.

────────────────────────────────────────────

11. basic.idc / test.idc
Purpose: Provide simple branching and loop examples for early dev.

Script Overview:
- Uses hardcoded conditionals and infinite loop with break logic.
- Verifies load/respond/delete in context of basic script structure.

Process:
First-pass programs used to validate that execution engine can parse, branch, and evaluate minimal logic trees.

────────────────────────────────────────────

12. test_conditionals.idc
Purpose: Confirm accurate `if`/`else` resolution based on memory input.

Script Overview:
- Performs two conditional checks: one `true`, one `false`.
- Confirms only correct branch executes and `else` is skipped/entered as needed.

Process:
Validates `last_if_result` internal state and conditional gatekeeping logic.

────────────────────────────────────────────

13. test_modifiers.idc
Purpose: Confirms accurate parsing of modifiers and memory lifetimes.

Script Overview:
- Loads values using `from`, creates and deletes memory keys.
- Verifies correct read/write access and deletion reporting.

Process:
Ensures modifier-based memory operations behave consistently in sequence.

────────────────────────────────────────────

14. test_flow_control.idc
Purpose: Stress test for loop iteration, break, and continue edge cases.

Script Overview:
- Executes three loops: one breaks immediately, one counts to 3, one exits at `2`.
- Verifies memory counters, break states, and dynamic conditional exits.

Process:
Tests interpreter’s loop stack, dynamic branching, and mid-loop memory interactions.

────────────────────────────────────────────

15.Pre-flight Check 
Purpose: This script performs a comprehensive test of the IDC interpreter, verifying that all critical components are functioning as expected and ready for production use.

Script Overview:

Memory Initialization: Verifies the interpreter's ability to load variables and perform arithmetic operations.

Arithmetic Testing: Ensures correct evaluation of expressions, including valid and invalid math operations.

Conditionals & Flow Control: Tests conditional statements, including the if and else branches based on memory values.

Loop & Memory Management: Validates the proper use of loops and memory operations (e.g., load, delete).

Error Handling: Ensures the interpreter gracefully handles errors like division by zero, mismatched parentheses, and invalid characters.

Stubbed Tokens: Checks the interpreter's ability to handle recognized but unimplemented tokens without crashing.

Process: This script was designed to test a wide range of interpreter capabilities and ensure that all components, from arithmetic to flow control, are working seamlessly. It helps confirm the overall stability of the interpreter before deployment.

────────────────────────────────────────────

16.test_integrations.idc
Purpose: This script tests the integration of various features in the IDC interpreter, including memory handling, conditional logic, error handling, and event-based actions like on, every, and route. It also checks for the correct execution of operations like loading, arithmetic, memory deletion, and handling unsupported or unimplemented tokens.

Key Features Tested:

Memory Operations:

Loads values into memory (x, y, sum), and performs arithmetic to check for correct handling.
Deletes specific memory entries and verifies that only the expected entries remain in memory.
Arithmetic Evaluation:Performs basic arithmetic (addition of x and y), handles division by zero, and resolves mismatched parentheses.
Conditional Logic:Verifies that the correct branches are executed based on the value of the sum variable.
Event Handling:Triggers an on event, checks the behavior of the every event (with periodic execution), and validates routing events using route.
Error Handling: Simulates invalid math operations (division by zero, mismatched parentheses) and ensures the interpreter handles these gracefully without crashing.
Stubbed Token Handling: Logs "stubbed" messages for unimplemented tokens, verifying that unsupported tokens are recognized but do not break execution.

Expected Behavior: The script should load values, perform the arithmetic operations correctly, trigger event actions, handle math errors appropriately, and demonstrate correct memory management.

The interpreter will log messages indicating when unsupported tokens are encountered but should continue execution without failure.



────────────────────────────────────────────
Core Interpreter Feature Tests
────────────────────────────────────────────

17. test_all_features.idc
Purpose: This comprehensive script validates the full functionality of the IDC interpreter by testing every major capability.

Script Overview:
- Arithmetic Logic: Loads values into `a` and `b`, then computes `sum` using `{{a}} + {{b}}`. Responds with the computed value.
- Conditional Check: If `sum` equals 5, responds with a success message. Else block is skipped correctly.
- Loop Execution: Uses a `while` loop with a counter to ensure arithmetic and memory mutation in loops function correctly.
- Memory Cleanup: Deletes `sum` and `counter`, leaving only `a` and `b` in memory.
- Token Coverage: Invokes every known stubbed token in the language to confirm 40-token coverage and correct fallback behavior.

Process:
This was built as a top-down integration test for interpreter stability. It confirms flow control, memory management, dynamic substitution, and fallback on unimplemented nodes.

────────────────────────────────────────────

18. a.idc
Purpose: Event block syntax test with response timing

Script Overview:
- on start / end / respond_event block execution
- every block timing and loop
- wait block delay
- route block and memory mutation
- Memory substitution in string outputs

Process:
Validates block event syntax and sequential memory usage

────────────────────────────────────────────

19. test_all.idc
Purpose: End-to-end integration test for runtime logic

Script Overview:
- load and respond using memory
- respond_event with math expression
- wait delay
- every loop (limited run)
- route logic
- on start / end sequencing
- Memory echo output

Process:
Confirms all key nodes work correctly in sequence



────────────────────────────────────────────
Usage
────────────────────────────────────────────

Run from CLI using:

    cargo run -- --run examples/<filename>.idc

Other phases:

    cargo run -- --tokens examples/<file>.idc
    cargo run -- --ast examples/<file>.idc
    cargo run -- --semantics examples/<file>.idc

────────────────────────────────────────────
Stubbed Token Notes
────────────────────────────────────────────

The following commands are fully parsed and logged, but currently stubbed:

    remember, get, capture, define, alias,
    print, log, show, alert, stop, retry, then,
    authorize, purge, reset, connect, disconnect,
    save, update, check, verify, use, from, to,
    with, as, every, route

They are included in flow and semantic analysis, and will be implemented in future phases.