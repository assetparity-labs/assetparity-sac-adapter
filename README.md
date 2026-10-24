# Stellar Asset Contract adapter

AssetParity's SAC route builds unsigned built-in SAC `transfer` calls and normalizes their observations. This repository does not deploy a custom token implementation or wrap an asset into a second token. The pilot uses Stellar's built-in asset contract for the same issued asset as the classic route.

## Two validation layers

- `index.mjs`: operation construction, explicit contract-error parsing and conservative outcome categories. Depends on the sibling classic adapter for exact base-unit handling. Install SDK dependencies with `npm ci` before building operations.
- `lib.rs`: native Rust host conformance tests using soroban-sdk 29.0.1. Tests cover exact fractional transfers, a single base unit, insufficient balance and missing spender authorization. These host tests use generated contract addresses; live account-trustline behavior is covered separately by the public testnet pilot.

```sh
node --test test.mjs
cargo test --locked
```

## Interpret errors carefully

SAC error 10 is a broad balance-constraint failure, not a unique insufficient-funds diagnosis. Codes 11 and 13 identify deauthorization and missing trustlines respectively. The adapter only extracts an explicit `Error(Contract, #N)` value; it does not guess contract codes from arbitrary host failures. A rejected RPC simulation has no charged transaction fee and is never labeled as an included failed transaction.

[SAC reference and error definitions](https://developers.stellar.org/docs/tokens/stellar-asset-contract). Integrated setup and testnet evidence live in [assetparity-backend](https://github.com/assetparity-labs/assetparity-backend).
