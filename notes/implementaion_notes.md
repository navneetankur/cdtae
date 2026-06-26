# Implementation Notes
```mermaid
flowchart LR
Start(( )) --input--> Parser --Command--> Executor --Updates--> S[State Machine]
````

## 1. Command Enum

The parser produces a Command enum.

Example:

```rust
enum Command {
    Write(String),
    Replace {
        old: String,
        new: String,
    },
    Undo,
    Redo,
    Interrupt,
}
```

---

## 2. Parser → Command

The parser converts user input into Commands.

Implementation is intentionally simple.

A handwritten parser is preferred initially.

Parser generators may be evaluated later if grammar complexity grows.

---

## 3. Executor(Command, State)

Execution logic is independent of parsing.

```text
Command
↓
Executor
↓
State change
```

---

## 4. Undo / Redo

Undo correctness is more important than sophistication.

Prefer a simple, reliable implementation over a clever one.

A snapshot-based history is acceptable.

Each resolution (command or flush-as-Write) is one history entry. Undo always recovers a full resolution, never an individual input.

---

## 5. Keep Voice in Mind

Speech is the primary design constraint.

Keyboard input is only the development and testing interface.

When choosing between:

```text
better keyboard UX
```

and

```text
better future speech UX
```

prefer the speech-friendly design unless complexity becomes unreasonable.

---

## 6. Input Terminology

The unit arriving from VAD is an **input**, not a sentence.

For v1, assume input is a sentence and will create a command.

A command may span multiple inputs (in V2).

Do not assume one input = one complete thought.

---

## 7. Incomplete Command Handling (V1)

An input that starts with a known verb but does not fully parse is incomplete.

Flush as Write. Recoverable with Undo.

Do not discard. Do not hold state.

---

## 8. Split Command Measurement (V1 → V2 gate)

Before building the V2 completion window, instrument VAD to measure how often commands arrive split across inputs.

If splits are rare, V1 flush-as-Write + Undo is sufficient.

If splits are common, implement V2 window.

---

## 9. Completion Window (V2 — deferred)

Implement only after measurement in §8 confirms splits are frequent.

See `docs/overview.md` V2 section for mechanism and resolution rules.

Key invariants:
- Buffer + re-parse on each new input (same stateless parser, no new grammar).
- All-or-nothing resolution: full parse → execute; failure or timeout → flush entire buffer as one Write.
- Interrupt → discard (only legitimate discard case).
- One Undo per resolution, always.
