### COMING SOON ###


# 🛠 IDC Debugging Guide: From Newbie to 1337 Hacker

This guide explains how to debug IDC programs using the built-in `explain:`, `why:`, and `trace:` introspection tools—tailored for every experience level.

---

##  For No-Coders & Beginners

###  Goal:
Understand what your program is doing, and how to ask it *why* it did something unexpected.

###  Simple Workflow:
1. Run your `.idc` program.
2. If something feels off, ask:
   ```idc
   explain: last response
   ```
3. If it responded something you didn’t expect, ask:
   ```idc
   why: did it respond "Welcome"
   ```
4. If something didn’t happen:
   ```idc
   why: did it skip "respond Unauthorized"
   ```

###  Tip:
Use `respond`, `remember`, `if`, and `stop` with clarity. Ask questions exactly the way you wrote them.

---

##  For Junior Developers

###  Goal:
Diagnose missing, skipped, or unexpected logic paths and fix flow issues.

###  Queries to Use:
- See execution after a key node:
  ```idc
  trace: logic after verify token
  ```
- Diagnose conflicting logic:
  ```idc
  why: did it respond "Welcome" and "Unauthorized"
  ```
- Trace memory keys:
  ```idc
  trace: memory from route "/login"
  ```

### 💡 Tip:
Make sure `stop`, `else`, and `retry` are used where needed to control flow.

---

##  For Mid-Level Coders

###  Goal:
Track execution chains, detect fallback misuse, and resolve logic drift across scopes.

###  Advanced Queries:
```idc
why: was fallback triggered in route "/reset"
explain: memory overwrite of "email_token"
trace: node path from route "/register"
```

###  Tip:
Use chaining (`→`) with awareness. If two nodes run and they shouldn’t, check for missing `stop`.

---

## For Senior Engineers / Architects

###  Goal:
Diagnose deeply nested behavior, reason about intention drift, and optimize structure.

###  Power Queries:
```idc
trace: memory access of "session_id" across all routes
why: did fallback not activate on token error
explain: condition tree of user verification logic
```

###  Tip:
Check trace trees to see which modifiers (`with`, `as`, `unless`) were applied and whether they affected child logic.

---

##  For 1337 Hackers (Deep Logic Surfers)

###  Goal:
Push the boundaries of introspection, test logic illusions, analyze side effects and recursion flow.

###  High-Entropy Queries:
```idc
explain: result path of route "/admin" including all skipped logic
trace: recursive fallback activations across chained retries
why: did "respond OK" trigger despite early failure
```

###  Tip:
Use introspection to build mental models of how your tree mutated during execution. Combine trace depth with memory logs and scope chains.

---

##  Summary Table

| Level | Common Queries |
|-------|----------------|
| Beginner | `explain: last response`, `why: did it skip` |
| Junior | `trace: after`, `why: conflict`, `memory from` |
| Mid | `why: fallback`, `trace: route`, `memory overwrite` |
| Senior | `condition tree`, `token path`, `trace: scope` |
| 1337 | `trace: recursion`, `result path`, `skipped forks` |

---

##  Final Tip:
IDC doesn’t crash. It flows. So your job is not to find broken code—it’s to **ask the right question** about what it actually did.

Let the language answer you.