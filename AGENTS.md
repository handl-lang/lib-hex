# lib-hex

`hex` — Handl stdlib hex codec implementing the `Encoding` trait from
`lib-encoding`. Package: `.hnl` surface plus a const-only Rust shim crate (ops pruned post-parity).

## Layout

| path | role |
| --- | --- |
| `src/index.hnl`, `src/index_flat.hnl` | the `.hnl` interface |
| `src/encode.hnl`, `src/decode.hnl` | codec arrows |
| `src/hex.test.hnl` | `.hnl` tests |
| `src/lib.rs` | const-only shim re-exporting the `.hnl` sources |
| `Cargo.toml`, `xtask/` | Rust crate + vendored repo gate |
| `pkg.hnl`, `scripts/hnl-gate.sh` | manifest + `.hnl` gate |
| `workflows/gate.yaml` | CI — shared `handl-gates` campaign |

## Contract

- Implements `lib-encoding`'s `Encoding` trait — same contract as
  `lib-base64`; keep the two codecs symmetric.
- `.hnl` gated by `hnl-gate.sh`; the shim crate by `cargo xtask gate`.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
cargo xtask gate
```

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
