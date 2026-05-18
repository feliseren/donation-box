# Stellar Donation Box

Stellar Donation Box adalah smart contract donasi sederhana yang dibangun menggunakan Soroban di Stellar Testnet.

Project ini memungkinkan user membuat campaign donasi, menerima donasi token, melihat total donasi, menutup campaign, dan melakukan withdraw dana oleh owner campaign.

## Fitur

- Create campaign donasi
- Donate token ke campaign
- Melihat detail campaign
- Melihat total donasi user
- Withdraw dana oleh owner
- Close campaign sebagai soft delete

## CRUD Mapping

| CRUD | Function |
|---|---|
| Create | `create_campaign()` |
| Read | `get_campaign()`, `get_available_balance()`, `get_donated_by_user()` |
| Update | `donate()`, `withdraw()` |
| Delete | `close_campaign()` sebagai soft delete |

## Tech Stack

- Stellar
- Soroban Smart Contract
- Rust
- Stellar CLI
- Stellar Testnet

## Build Contract

```bash
stellar contract build
