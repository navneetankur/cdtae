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

[Mel-specs](https://github.com/wavey-ai/mel-spec/tree/main/examples/stream_whisper) gives us a way to turn streaming voice into text. It will do Voice Activity Detection (VAD). Processes audio in chunks and transcribes when speech boundaries are detected. There can be silence in between human speech of words. So not really full sentence but we can assume a reasonable collection of words. We will call it **input**. VAD splits on silence; a single command may arrive across multiple inputs. So we can't always assume one input == one thought.

This document is wip, and nothing is final. We are not even v1. we are still exploring and open to change if new requirements surfaces, or better idea/way arrives.

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

### Incomplete Replace

An input starting with "replace" but missing the full `replace X with Y` form does not parse.

It is flushed as a Write (the whole incomplete input becomes literal text).

This is recoverable with Undo.

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

# V1 Parser Behavior

- The unit arriving from VAD is an **input**, not a sentence. Do not assume one input = one complete thought.
- Default mode is Write. Anything not recognized as a command is appended as text.
- A command must start with a known verb **and** parse completely within one input to execute.
- An input that starts with a known verb but does not parse completely is **incomplete**. It is flushed as a Write. Recoverable with Undo.
- Nothing is silently discarded in V1.

---

# V1.2

### Interrupt

Abort the current operation.

Exact behavior TBD.

Motivation:

Future speech workflows may need a way to cancel actions or partially completed commands.

---

# V2 Completion Window

Implement only after measuring how often commands arrive split across inputs in V1.

If measurement shows splits are common:

### Mechanism

Buffer the pending inputs and re-run the same stateless parser on the concatenated text after each new input arrives.

### Resolution — all or nothing

* Full parse → execute the command. One history entry.
* Structurally incompatible input arrives → flush the whole buffer as one Write. One history entry.
* Timeout → flush as Write.
* Interrupt → discard. The only case where discard is correct; it is explicit and user-initiated.

One Undo always recovers the full resolution, never individual inputs.

### Why all-or-nothing

Individual inputs are never resolved separately. On failure, the entire buffered group (including the initial verb input) becomes one Write. This avoids silent loss and keeps Undo simple.

### Defer

* Timeout length.
* Literal escape for dictating command words as content (e.g. "replace box with fox" as text, not a command). Needs a delimited quote/unquote idiom — not a mode switch.
* some way to show current partial internal state. So user can decide to continue or interrupt.

---

# Open Questions

O1 — Measure split frequency before building V2 window. Instrument VAD in V1.

O2 — Greedy commit: commit the instant a command fully parses. A multi-word target split across inputs (e.g. "with fox" then "terrier") commits "fox" early — rare and recoverable, accept it for now.

O3 — Timeout length for V2 window.

O4 — Literal escape idiom for dictating command words as content. Delimited quote/unquote, not a mode switch.

O5 - what does "undo here is a fox" mean? Command(undo) and then Write(here is fox) or just Write(undo here is a fox)?
