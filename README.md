# lib-hex

`hex` — Handl stdlib codec. Implements the `Encoding` trait from
`lib-encoding`; the `.hnl` surface lives in `src/index.hnl`.

## Layout

```text
src/index.hnl   the interface
src/ops.rs      the Rust atoms
pkg.hnl         package manifest — modules, deps, description, license
workflows/gate.yaml           bupha entry (handl-gates)
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
