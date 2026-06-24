# ALens

ALens is a smart contract security analysis tool built in Rust.

## What it does

ALens analyzes Solidity smart contracts and generates a security report.

Current capabilities:

* Detects functions
* Detects state variables
* Flags withdrawal functions
* Flags centralized ownership patterns
* Flags balance storage patterns

## Example Output

```text
=== AUDIT REPORT ===

Functions Found:
- deposit
- withdraw

State Variables:
- owner
- balances

Findings:
[HIGH] Withdrawal function detected.
[MEDIUM] Centralized ownership detected.
[MEDIUM] Stores user funds. Review reentrancy protections.
```

## Built With

* Rust
* Cargo

## Future Improvements

* AI-powered vulnerability analysis
* Solana support
* Function call graph generation
* Risk scoring
* Web interface
