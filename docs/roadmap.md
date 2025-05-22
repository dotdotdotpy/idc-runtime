
#  IDC Project Roadmap

This roadmap outlines the full progression of the IDC ("I Don’t Code") language development across all nine phases, with supplemental inclusions to ensure all critical components are implemented on time.

---

##  Phase 1: Core Language Definition & Parser Design

- [x] Define the foundational 40-token lexicon
- [x] Write full grammar rules (sentence structures, chaining, conditionals)
- [x] Draft grammar spec (docs/spec.md) with examples
- [x] Create .idc example snippets and use cases
- [x] Define parser architecture (Rust structure, file layout)
- [x] Plan token system and AST node types
- [x] Approve syntax policies (no brackets, chaining format, memory rules)

 Output: `docs/spec.md`, `docs/lexicon.md`, `examples/*.idc`, `parser_plan.md`

---

##  Phase 2: Rust Interpreter Scaffold

- [x] Set up Rust project and folder structure
- [x] Implement tokenizer and token registry
- [x] Implement AST and grammar matcher
- [x] Build flow tree scaffold (ExecutionTree struct)
- [x] Connect parsed commands to logic nodes
- [x] Build first test parser with static sentences
- [x]  **Supplemental:** Default `respond` behavior
- [x]  **Supplemental:** CLI debug flags (`--tokens`, `--ast`, `--run`)

 Output: `src/parser/`, `src/interpreter/`, `tests/parser_tests.rs`

---

##  Phase 3: Runtime Engine & Memory System

- [x] Implement runtime loop
- [x] Add scoped memory + context store
- [x] Support chaining (`→`), retry logic, conditionals
- [x] Implement fallback handling + execution tracing
- [ ] Add base logging and debug introspection
- [x]  **Supplemental:** Token type metadata
- [ ]  **Supplemental:** explain:/why: tracer support
- [x]  **Supplemental:** Execution cap depth

 Output: `src/runtime.rs`, `src/memory.rs`, `src/utils/logger.rs`

---

##  Phase 4: Module System & Standard Library

- [ ] Define `mod.rs` for each stdlib module
- [ ] Implement built-in modules (io, logic, strings, time, etc.)
- [ ] Support module parameters and contextual behavior
- [ ] Enable chaining and output re-use across modules
- [ ] Create modular test cases for each stdlib function
- [ ]  **Supplemental:** Scoped runtime snapshots
- [ ]  **Supplemental:** Token expansion protocol

 Output: `src/stdlib/`, `tests/stdlib_tests.rs`

---

##  Phase 5: CLI Interface & Interactive Shell

- [x] Create `main.rs` runner for executing `.idc`
- [x] Build CLI interface (basic command runner)
- [ ] Implement REPL with input loop and live evaluation
- [ ] Add logging flags, memory dump, error trace modes
- [x] Handle basic file IO for `.idc` programs
- [ ]  **Supplemental:** Token inheritance/aliasing
- [ ]  **Supplemental:** Logic chain visualizer (optional)

 Output: `src/cli/`, `src/main.rs`, `examples/`, `docs/usage.md`

---

##  Phase 6: Error Handling, Permissions & Sandbox

- [ ] Add sandbox flags to sensitive modules
- [ ] Implement permission prompts (e.g. `authorize: behavior`)
- [ ] Add runtime error recovery system
- [ ] Support `explain:` and `why:` introspective queries
- [ ] Human-readable tracebacks and fallback explanations
- [ ]  **Supplemental:** Sandboxing rules
- [ ]  **Supplemental:** Plugin module format

 Output: `src/utils/error.rs`, `src/runtime/sandbox.rs`, `docs/safety.md`

---

##  Phase 7: Demo Programs & Showcase Content

- [x] Create real-world `.idc` apps (server, scheduler, automation)
- [ ] Write comparisons showing code reduction (e.g. 500:1 collapse)
- [x] Add README examples for GitHub
- [ ] Document module behavior in `docs/spec.md`
- [ ] Create 2–3  demos
- [ ]  **Supplemental:** Example expansion plan (10–20 more examples)

 Output: `examples/`, `docs/demo_showcase.md`, `README.md`

---

##  Phase 8: Web Playground (Optional)

- [ ] Port core to WASM (if needed)
- [ ] Build web UI + playground interface
- [ ] Add copy/paste examples + visual tree explorer
- [ ] Enable GPT integration in sandbox
- [ ] Add syntax highlighting + real-time feedback
- [ ]  **Supplemental:** Tutorial mapping system

 Output: `playground/`, `wasm/`, `web_runner.js`

---

##  Phase 9: Launch Prep & Publishing

- [ ] Write full docs: spec, glossary, usage, roadmap
- [ ] Finalize GitHub repo and licensing
- [ ] Launch v1 with clear goals for v2 (AI-powered features, plugins, IDE)
- [ ] Share to dev communities and AI-first coders
- [ ] Begin collecting feedback and contribution interest
- [ ]  **Supplemental:** Versioning header (`# IDC v1.0`)
- [ ]  **Supplemental:** Lexicon autodoc tool
- [ ]  **Supplemental:** Contribution policy (AI + human)

 Output: `docs/`, `LICENSE`, `github_release.md`

---
