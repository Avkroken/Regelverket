# YAML adaptation fidelity v0

Status: technology-spike evidence
Date: 2026-08-27

## Question

Can the Go and Rust technology candidates make a narrow semantic edit to a human-maintained GitHub Actions workflow without rewriting unrelated YAML?

The fixture deliberately contains comments, mixed quoting, flow-style YAML, aligned inline comments and a block scalar. Both implementations change only `env.RUNTIME` from `"20"` to `"22"`.

## Candidates

- Go: `github.com/grave0x/yamsplice` v0.3.0
- Rust: `yaml-rt` v0.2.3 with default features disabled

This is a library comparison inside the language spike, not a commitment to either dependency for production.

## Measured result

CI run `33099364114` captured the exact outputs.

Rust preserved the complete target line byte-for-byte except for the intended scalar value:

```text
  RUNTIME: "22"     # adaptation target; preserve quoting and this comment
```

Go preserved every untouched line byte-for-byte, preserved double quoting and the complete inline comment, but normalized the spacing before the inline comment on the edited line:

```text
  RUNTIME: "22" # adaptation target; preserve quoting and this comment
```

No unrelated line, comment, flow-style sequence, quote choice or block scalar changed in either output.

## Preservation policy used by the spike

For user-owned workflow adaptation, the test distinguishes two guarantees:

1. **Unrelated-byte preservation:** every line outside the edited node must remain byte-identical.
2. **Edited-node fidelity:** the intended semantic value, quote style and attached inline comment must be retained; formatting noise on the edited line is measured explicitly.

Rust currently satisfies the stronger byte-exact edited-line expectation for this fixture. Go/yamsplice satisfies unrelated-byte preservation but introduces one measured formatting normalization on the edited line.

## Decision impact

This is evidence in Rust's favor on YAML transformation fidelity, but it is not sufficient by itself to choose Rust. The technology gate still requires GitHub adapter/error-classification evidence, cross-platform packaging evidence and the final contributor/maintenance comparison.

A production implementation must also review dependency licensing, maintenance and supply-chain characteristics separately from fidelity behavior.
