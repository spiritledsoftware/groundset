# Domain Docs

This repo uses a **single-context** domain-doc layout.

## Before exploring, read these

- **`CONTEXT.md`** at the repo root.
- **`docs/adr/`**: read ADRs that touch the area you're about to work in.

If either location doesn't exist, proceed silently. The `/domain-modeling` skill creates documentation lazily when terms or decisions are resolved.

## File structure

```text
/
├── CONTEXT.md
├── docs/adr/
└── src/
```

## Use the glossary's vocabulary

When output names a domain concept—in an issue title, refactor proposal, hypothesis, or test name—use the term defined in `CONTEXT.md`. Don't drift to synonyms the glossary explicitly avoids.

If a needed concept isn't in the glossary, reconsider whether it belongs or note the gap for `/domain-modeling`.

## Flag ADR conflicts

If output contradicts an existing ADR, surface it explicitly rather than silently overriding:

> _Contradicts ADR-0007 (event-sourced orders), but worth reopening because…_
