#  IDC Glossary

This glossary defines key terms, grammar components, and runtime concepts used in the IDC language.

---

##  Core Concepts

- **IDC**: "I Don’t Code" — a natural-language-based programming language designed to be written by humans and interpreted by AI, especially ChatGPT.
- **Intention-Based Programming**: Writing desired outcomes, not step-by-step instructions.
- **Logic Tree**: The internal structure of all IDC programs, where each node represents an intention (action, condition, etc.).
- **Chaining (`→`)**: Connects logic steps linearly without brackets.
- **Fallback Logic**: Auto-handled `else`, `retry`, or soft default paths when conditions fail.
- **Scope**: Context of memory, values, and logic at a specific level of nesting or flow.

---

##  Token Types

| Type      | Description |
|-----------|-------------|
| Action    | Performs something (e.g. `load`, `log`, `respond`) |
| Condition | Checks truth (e.g. `if`, `unless`, `is`, `has`) |
| Modifier  | Alters behavior (e.g. `then`, `else`, `retry`, `with`) |
| Event     | Starts logic on triggers (e.g. `on`, `every`) |
| Memory    | Stores, retrieves, or renames values (e.g. `remember`, `get`, `as`) |
| System    | Permission or state control (e.g. `authorize`, `purge`) |
| Flow      | Guides execution flow (`stop`, `continue`, `use`) |
| Entity    | Defines logic blocks (`route`) |

---

##  Execution Terms

- **Node**: A single unit of logic inside the execution tree.
- **Chain**: A sequence of operations linked by `→`.
- **Children**: Nested logic underneath a condition or action.
- **Modifier**: Adds meaning to a base token (`with`, `as`, `from`, etc.).
- **Fallback Path**: What executes if the previous step fails.

---

##  Syntax Conventions

- Sentences start with a **token** from the Foundational 40.
- No brackets. Nesting is handled by indentation and chaining.
- Commas = soft pauses. Arrows = strict chaining.
- Conditionals (`if`, `unless`) consume the next line if no action is present.

---

##  Example Sentence

```
on user login → verify token → if invalid → respond "Unauthorized" with status 401
```

- **Event trigger**: `on user login`
- **Chained logic**: `verify token → if invalid → respond`
- **Modifier**: `with status 401`