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

[mel-specs](https://github.com/wavey-ai/mel-spec/tree/main/examples/stream_whisper) gives us a way to turn streaming voice into text. It will do Voice Activity Detection (VAD). Processes audio in chunks and transcribes when speech boundaries are detected. So we can assume that we will get full sentence.

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
