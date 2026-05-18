# Stellar Donation Box

Stellar Donation Box is a simple donation smart contract built on the Stellar Testnet using Soroban.

This project was created for the Rise In x Stellar workshop. It allows users to create donation campaigns, donate Stellar-based tokens, check campaign details, withdraw funds as the campaign owner, and close campaigns.

## Deployed Smart Contract

```txt
CCHXTE7GP2M6RPJACBH7O6YHOX5O24VNTAYHZV3UMZBMD7BRGYCILHGO
```

## Project Overview

Stellar Donation Box is a transparent donation campaign smart contract.

Each campaign stores:

- Campaign owner
- Token contract address
- Campaign title
- Total donated amount
- Total withdrawn amount
- Campaign status

## Features

- Create a donation campaign
- Donate tokens to a campaign
- View campaign details
- View available campaign balance
- View total donation by a specific user
- Withdraw funds as the campaign owner
- Close a campaign as a soft delete

## Tech Stack

- Stellar
- Soroban Smart Contract
- Rust
- Stellar CLI
- Stellar Testnet

## Smart Contract Path

```txt
contracts/notes/src/lib.rs
```

## CRUD Mapping

| CRUD | Function | Description |
|---|---|---|
| Create | `create_campaign()` | Creates a new donation campaign |
| Read | `get_campaign()` | Reads campaign details |
| Read | `get_available_balance()` | Reads available campaign balance |
| Read | `get_donated_by_user()` | Reads total donation from a specific user |
| Update | `donate()` | Adds donation to a campaign |
| Update | `withdraw()` | Allows owner to withdraw funds |
| Delete | `close_campaign()` | Closes campaign as a soft delete |

## Contract Functions

### `create_campaign`

Creates a new donation campaign.

Parameters:

```txt
owner: Address
token: Address
title: String
```

Returns:

```txt
campaign_id: u32
```

### `donate`

Transfers tokens from a donor to the Donation Box smart contract.

Parameters:

```txt
campaign_id: u32
donor: Address
amount: i128
```

### `withdraw`

Allows the campaign owner to withdraw funds from the campaign.

Parameters:

```txt
campaign_id: u32
to: Address
amount: i128
```

### `close_campaign`

Closes the campaign so it can no longer receive donations.

Parameters:

```txt
campaign_id: u32
```

### `get_campaign`

Returns campaign details by campaign ID.

Parameters:

```txt
campaign_id: u32
```

### `get_donated_by_user`

Returns the total amount donated by a specific user.

Parameters:

```txt
campaign_id: u32
user: Address
```

### `get_available_balance`

Returns the remaining campaign balance that has not been withdrawn.

Parameters:

```txt
campaign_id: u32
```

## Build the Contract

```bash
stellar contract build
```

Expected result:

```txt
Build Complete
Wasm File: /app/target/wasm32v1-none/release/donation_box.wasm
```

## Generate a Testnet Account

```bash
stellar keys generate eren --network testnet --fund
```

Check the account address:

```bash
stellar keys address eren
```

## Get Native XLM Token Contract ID

```bash
stellar contract id asset --network testnet --asset native
```

Save the output as `TOKEN_ID`.

## Deploy the Contract

```bash
stellar contract deploy --wasm /app/target/wasm32v1-none/release/donation_box.wasm --source-account eren --network testnet
```

Deployed contract address:

```txt
CCHXTE7GP2M6RPJACBH7O6YHOX5O24VNTAYHZV3UMZBMD7BRGYCILHGO
```

```

## Author

Name: Felis Eren Cristi Milala 
Workshop: Rise In x Stellar  
Project: Stellar Donation Box
