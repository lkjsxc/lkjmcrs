# Commit Policy

## Frequency

- Commit after each coherent verified batch.
- Prefer small batches with one clear purpose.
- Land docs-only batches before dependent code batches.
- Do not collect unrelated work into a large final commit.

## Preconditions

- Relevant checks pass or the batch is docs-only and structurally inspected.
- Commit message describes changed contracts or behavior.
- `git status` is reviewed before committing.
