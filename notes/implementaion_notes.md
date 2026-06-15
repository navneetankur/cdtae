# Implementation Notes

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
    Show,
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
