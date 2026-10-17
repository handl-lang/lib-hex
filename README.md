# lib-hex

`hex` — Handl stdlib codec. Implements the `Encoding` trait from
`lib-encoding`; the `.hnl` surface lives in `src/index.hnl`.

Part of the stdlib → `lib-*` split (ADR-0128). History is a `git subtree split`
of `stdlib/handl-packages/hex/`.

```text
src/index.hnl   the interface
src/ops.rs      the Rust atoms
pkg.hnl         { modules: ["hex"] }
ci.sh           bupha entry
```
