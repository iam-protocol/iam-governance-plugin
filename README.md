# IAM Voter Weight Plugin

SPL Governance voter weight plugin for [IAM Protocol](https://iam-human.io). Gates DAO voting on behavioral proof-of-humanity verification.

## What it does

DAOs on [Realms](https://app.realms.today) can configure this plugin to require members to have a valid IAM Anchor with sufficient Trust Score before casting votes, creating proposals, or participating in governance.

One person, one vote, verified through behavioral consistency over time. No face scans. No document checks. No hardware.

## How it works

1. DAO admin calls `create_registrar` with a minimum Trust Score and maximum verification age
2. Each voter calls `create_voter_weight_record` to initialize their record
3. Before voting, the voter calls `update_voter_weight_record` which reads their IAM IdentityState PDA cross-program and checks:
   - Trust Score >= minimum configured by the DAO
   - Last verification is recent enough (within max_verification_age seconds)
4. If both pass, voter_weight is set to 1 (one person, one vote) with a short expiry (~40 seconds)
5. The governance program reads the VoterWeightRecord and allows the vote

The voter weight expires after ~100 slots, forcing the update to happen in the same transaction as the governance action. This prevents stale weight records from being reused.

## Architecture

```
Voter wants to cast a vote
    → calls update_voter_weight_record
    → plugin reads IAM IdentityState PDA (cross-program, no CPI)
    → checks trust_score >= min_trust_score
    → checks verification age < max_verification_age
    → sets voter_weight = 1, expiry = current_slot + 100
    → governance program reads VoterWeightRecord
    → vote is accepted
```

The plugin reads the IAM IdentityState account via raw byte deserialization (not Anchor CPI) to avoid version coupling with the IAM Anchor program.

## Program ID

**Devnet:** `99nwXzcugse3x8kxE9v6mxZiq8T9gHDoznaaG6qcw534`

## Build

```bash
anchor build
```

Requires:
- Anchor 0.31.1
- Solana CLI 2.2.1
- Rust 1.91.0

## Test

```bash
anchor test
```

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| anchor-lang | 0.31.1 | Anchor framework |
| spl-governance-mythic | 3.1.2 | Realm data validation |
| spl-governance-addin-api-mythic | 0.1.6 | VoterWeightRecord type |
| solana-program | 2.2.1 | Solana runtime |

## Related

- [IAM Protocol](https://iam-human.io) -- behavioral proof-of-humanity on Solana
- [Pulse SDK](https://www.npmjs.com/package/@iam-protocol/pulse-sdk) -- client-side verification SDK
- [Protocol Core](https://github.com/iam-protocol/protocol-core) -- on-chain identity programs
- [Governance Program Library](https://github.com/Mythic-Project/governance-program-library) -- reference voter weight plugins

## License

MIT
