#  IDC Runtime Modes (V2 IDEA PITCH)

This document outlines the two supported runtime modes for IDC: a fully offline, self-contained execution mode, and an optional AI-augmented mode for future integration.

---

##  1. UNPLUGGED RUNTIME (Default)

**Description:**  
This is the default mode of IDC. It runs entirely offline and does not rely on any AI systems, APIs, or internet access. All program logic, interpretation, memory handling, introspection, and debugging features are handled directly by the interpreter.

### Features:
- Full support for all Foundational 40 tokens
- Natural language grammar & logic chaining
- Scoped memory and fallback execution
- Deterministic logic trees and trace logs
- Native support for:
  - `respond`
  - `remember`, `get`, `as`
  - `if`, `else`, `unless`
  - `route`, `on`, `every`
  - `stop`, `retry`, `continue`
  - `explain:`, `why:`, `trace:`

### Capabilities:
- Self-contained CLI and REPL
- No API keys or subscriptions required
- Can run in air-gapped environments
- Explain and trace logic without AI assistance

### Use Case:
This mode is ideal for developers, power users, and embedded systems where AI is unnecessary or undesirable.

---

##  2. AI-AUGMENTED RUNTIME (Optional Future Mode)

**Description:**  
In this optional future mode, IDC integrates with an AI model (e.g., ChatGPT or other LLMs) to provide deeper reasoning, debugging help, and natural language interaction. This is **not part of the core Phase 1–9 roadmap**, and would be introduced in a future release (e.g., Phase 10 or a plugin system).

### Features:
- Context-aware analysis of logic trees
- Conversational debugging: “Why didn’t this respond?”
- Suggest fixes, rewrites, or improvements
- Memory-aware introspection
- Optional use of `suggest:`, `rewrite:`, or natural prompts

### Requirements:
- API key (e.g., OpenAI)
- Internet connection
- Opt-in flag or REPL toggle (e.g. `--use-ai`, `copilot on`)

### Use Case:
Best for:
- New coders unfamiliar with logic modeling
- AI-driven assistant tooling
- Rich debugging UX in editors or online IDEs

---

##  Notes

- **Current Build Target:** UNPLUGGED RUNTIME ONLY
- AI integration is **not active** and **not assumed** in any roadmap or system behavior.
- This document simply defines the future possibility for optional augmentation.