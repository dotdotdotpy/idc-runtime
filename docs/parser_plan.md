# IDC Parser Plan

This document outlines the plan for implementing the IDC parser. It explains the parser’s responsibilities, the structure of its output (AST), and the Rust files that will be used to build its internal architecture.

---

## 1. What the Parser Should Do (Plain Language)

The IDC parser reads each `.idc` file line by line and transforms the natural-language logic into an internal structure called an Abstract Syntax Tree (AST). Its responsibilities include:

- Tokenizing each line (e.g. detecting keywords like `on`, `if`, `respond`)
- Validating sentence structure according to IDC grammar rules
- Building a chainable logic tree from chained (`→`) or indented logic
- Identifying and recovering from common grammar mistakes
- Mapping intent (e.g. “on login → if missing → respond”) into clear, structured operations
- Handling fallback rules (e.g. missing `then`, implicit `respond`)
- Tracking indentation depth for scoping logic
- Providing debug-friendly output for errors, misalignment, or skipped logic

---

## 2. AST Output Structure

The parser will convert lines into a nested logic structure using nodes. Each node will represent an operation or condition, and will be connected to its children through chaining or indentation.

### Basic AST Node Format
```json
{
  "type": "Action" | "Condition" | "Event" | "Modifier",
  "token": "respond",
  "args": [""OK""],
  "modifiers": {
    "status": 200
  },
  "children": [ ... ]  // Chained or indented logic
}
```

### Example: on user login → verify token → if invalid, respond "Invalid token"
```json
{
  "type": "Event",
  "token": "on",
  "args": ["user login"],
  "children": [
    {
      "type": "Action",
      "token": "verify",
      "args": ["token"],
      "children": [
        {
          "type": "Condition",
          "token": "if",
          "args": ["invalid"],
          "children": [
            {
              "type": "Action",
              "token": "respond",
              "args": [""Invalid token""],
              "modifiers": {
                "status": 401
              }
            }
          ]
        }
      ]
    }
  ]
}
```

---

## 3. Parser File Architecture (Planned Rust Modules)

### 📁 `src/parser/`
- **`tokenizer.rs`** – Breaks lines into tokens and segments (token + args + modifiers)
- **`grammar.rs`** – Applies sentence pattern recognition rules to parsed tokens
- **`ast.rs`** – Constructs and returns the full abstract syntax tree (AST)
- **`semantic.rs`** – Checks meaning, scope, and fallback logic inside trees
- **`parser.rs`** – Public-facing parser entry point that coordinates all steps

---

## 4. Additional Notes

- The parser should be able to parse partial or broken logic and issue soft warnings, not hard failures
- Chains (`→`) will be collapsed into a `children` array during AST formation
- Indented lines will be added as children of the last active block
- If no `respond` is detected in a command context, one may be auto-injected
- Parser will log recovery behavior and optionally output debug ASTs

---

This plan serves as the foundation for the interpreter and execution engine to be built in Phase 2.
