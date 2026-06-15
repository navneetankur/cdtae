# CDTAE

## What is it?

CDTAE is a speech-first text amendment engine.

The primary use case is:

```text
voice
↓
speech-to-text
↓
wrong words
↓
voice corrections
↓
final text
```

The goal is not code editing.

The goal is writing:

* notes
* emails
* messages
* fiction
* documents

using speech, while correcting transcription mistakes without reaching for a keyboard.

Speech recognition is intentionally outside the scope of CDTAE.

CDTAE operates on text.

---

## Terminology

The word "command" is overloaded.

Internally:

```text
User Input
↓
Parser
↓
Command
↓
Executor
```

A Command is an internal operation.

Examples:

```text
Write("The quick brown fox")
Replace("box", "fox")
Undo
Redo
```

Users do not need to think about Commands.

---

# V1

Default mode:

```text
Write
```

Anything not recognized as an editing instruction is treated as text.

Example:

```text
The quick brown box.
```

Appends text to the buffer.

---

## Editing Instructions

### Replace

```text
replace box with fox
```

Replace all occurrences.

Exact matching only.

---

### Undo

```text
undo
```

Revert the previous accepted change.

---

### Redo

```text
redo
```

Reapply an undone change.

---

# V1.2

### Show

```text
show
```

Display current buffer.

---

### Interrupt

Abort the current operation.

Exact behavior TBD.

Motivation:

Future speech workflows may need a way to cancel actions or partially completed commands.

---

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
