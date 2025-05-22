
# IDC Grammar Rules

This section defines how IDC sentences are structured using the foundational 40 tokens. The grammar is designed to be intuitive, human-readable, and chaining-friendly without traditional syntax like brackets or semicolons.

---

## 1. Sentence Structure Patterns

Each line in IDC is a sentence that maps to one or more logic instructions. Common grammar forms:

### Basic Action
```
[action] [target]
```
Example:
```
load config from "./settings.json"
```

### Event Trigger
```
on [event], [action]
```
Example:
```
on user login, verify token
```

### Conditional Branch
```
if [condition], [action]
```
Example:
```
if token is expired, redirect to login
```

### Chained Logic
```
[action] → [next action] → [next action]
```
Example:
```
load user → verify token → respond user
```

### Conditional with Fallback
```
if [condition], [action] → else [fallback action]
```
Example:
```
if error, retry once → else notify admin
```

### Timed Event
```
every [duration], [action]
```
Example:
```
every day at 3am, backup database
```

### Scoped Memory Storage
```
remember [value] as [name]
```
Example:
```
remember query.id as user_id
```

### Response with Modifier
```
respond [value] with [modifier]
```
Example:
```
respond "Missing ID" with status 400
```

---

## 2. Optional Elements

Some sentence parts are optional but help refine behavior:

- `→` chaining is optional for linear logic
- `then` and `else` act as flow modifiers
- `retry` can be attached after a failed action
- `with`, `as`, `from`, and `to` clarify input/output paths

Example with optional chaining:
```
check login → if failed, retry once → else continue
```

---

## 3. Chaining Instead of Brackets

IDC avoids brackets entirely. All nesting is implied by:

- Indentation (when multiline)
- Arrow (`→`) chaining

Instead of:
```javascript
if (error) {
  retry();
} else {
  alert("failed");
}
```

You write:
```
if error, retry once → else alert "failed"
```

---

## 4. Fallback Rules

IDC is designed to recover from missing grammar in a smart way.

- If `respond` is used alone: assume status 200
- If `if` has no `then`, apply the next line as the action
- If an action lacks a target (e.g. `log`), assume last context
- If `→` ends a line, assume the next line is a continuation

Examples:
```
respond "Done"         // returns 200 OK
if not found           // next line is fallback
  respond 404
```

---

## 5. Line Continuation Rules

When writing multi-line chains:
- Indent after events, `→`, or conditionals
- The first word of an indented line must be an action or modifier

Example:
```
on user signup
  check email → if invalid
    respond "Invalid email" with status 400
  else
    remember email as verified
    respond "Success"
```

---

## 6. Dynamic Scope Binding

All actions use the current scope unless overridden:
- `load` affects global memory unless saved to a name
- `respond` targets the latest user/session
- `log` appends to default log unless `to` is given

---

## 7. Visual Syntax Guide

This section demonstrates how IDC logic flows through tokens visually and semantically.

### Example 1
```
on user login, verify token → if invalid, respond "Invalid token" with status 401
```

- `on user login` → Event trigger
- `verify token` → Action
- `if invalid` → Conditional check
- `respond ... with status` → Action + modifier
- `→` → Chained sequence

Plain English:
> When a user logs in, verify their token. If it's invalid, return a 401 error.

---

### Example 2
```
every hour
  load stats from server → save to cache → log success
```

- `every hour` → Recurring event
- `load ... from` → Action pulling from source
- `save to cache` → Storage action with destination
- `log success` → Notification action

Plain English:
> Every hour, grab fresh stats, store them in the cache, and record that it worked.

---

### Example 3
```
if session missing
  respond "Not logged in" with status 403
else
  continue
```

- `if ...` → Condition
- `respond` → Action with output
- `else` → Fallback
- `continue` → Flow command

Plain English:
> If there's no session, tell the user they can't access the resource. Otherwise, move forward.

---

## 8. Syntax Policy

This section formally locks the syntax design of IDC as of Phase 1 completion. These rules ensure consistent parsing, readability, and future-proofing of the language.

### Approved Syntax Design
- No brackets (e.g. `{}`, `()`) are allowed — logic flows linearly
- Use `→` for chaining actions or modifiers
- Indentation defines scope, not braces or explicit nesting

---

### Final Syntax Rules (Hard Commit)

- Each sentence starts with a keyword from the Foundational 40 (e.g. `on`, `if`, `load`, `respond`)
- Actions and modifiers follow in natural-language order
- Commas `,` are allowed for separating conditions or soft pauses
- Chaining with `→` is optional but recommended for clarity
- `else`, `then`, `retry` are valid mid-chain operators
- Indentation must be consistent (spaces or tabs, not both)

---

### Edge Case Handling

The following behavior applies when the parser encounters edge conditions:

| Condition | Parser Behavior |
|----------|------------------|
| Missing `respond` | Default to `respond "OK"` with status 200 |
| `if` with no inline action | Use next indented line as the conditional result |
| `→` ends a line | Assume next indented line is the next step in chain |
| `log`, `alert`, etc. with no destination | Use last known context or global fallback |
| Missing `then` | Infer `→` as continuation step |
| Unrecognized token | Warn in console, skip logic but do not crash |

---

### Syntax Lock Justification

These rules balance:
- Intuitive writing for users
- Reliable parsing for the compiler
- Recoverability when mistakes happen
- Minimalism for an AI-native scripting model

Future expansions (e.g. macros, async handlers, inline expressions) will respect this core shape.
