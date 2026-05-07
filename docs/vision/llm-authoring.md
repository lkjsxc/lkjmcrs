# LLM Authoring Rules

## Formatting Rules

- Use stable headings such as `Goal`, `Rules`, `Contract`, and `Verification`.
- Keep one requirement per bullet when possible.
- Use relative links only inside docs.
- Prefer short declarative bullets over long narrative paragraphs.
- Put exact defaults, command names, protocol targets, and paths in code spans.

## Topology Rules

- Every docs directory has exactly one `README.md`.
- Every docs directory has multiple children besides `README.md`.
- Parent `README.md` files must change in the same batch as child additions.
- Split by canonical ownership, not by arbitrary line count alone.

## Change Anatomy

1. Find or create the owner doc.
2. Update related docs only where they add non-duplicate constraints.
3. Implement the documented behavior.
4. Update verification gates for regressions that matter.
5. Commit the coherent verified batch.

## Length Rules

- Docs files stay at `<= 300` lines.
- Authored source files stay at `<= 200` lines.
- Do not compress code or prose in a way that harms readability.
