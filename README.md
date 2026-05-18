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

## How to Interact with the Contract

Before running the commands below, replace:

```txt
EREN_ADDRESS_HERE
```

with your Stellar account address.

Replace:

```txt
TOKEN_ID_HERE
```

with the native XLM token contract ID from Stellar Testnet.

## 1. Create Campaign

```bash
stellar contract invoke --id CCHXTE7GP2M6RPJACBH7O6YHOX5O24VNTAYHZV3UMZBMD7BRGYCILHGO --source-account eren --network testnet -- create_campaign --owner EREN_ADDRESS_HERE --token TOKEN_ID_HERE --title "Flood Relief Donation"
```

Expected output:

```txt
0
```

This means the campaign was created with `campaign_id = 0`.

## 2. Get Campaign Details

```bash
stellar contract invoke --id CCHXTE7GP2M6RPJACBH7O6YHOX5O24VNTAYHZV3UMZBMD7BRGYCILHGO --source-account eren --network testnet -- get_campaign --campaign_id 0
```

Expected data:

```txt
owner
token
title
total_donated
total_withdrawn
active
```

## 3. Donate to Campaign

```bash
stellar contract invoke --id CCHXTE7GP2M6RPJACBH7O6YHOX5O24VNTAYHZV3UMZBMD7BRGYCILHGO --source-account eren --network testnet -- donate --campaign_id 0 --donor EREN_ADDRESS_HERE --amount 10000000
```

Note:

```txt
10000000 = 1 XLM if the token uses 7 decimals
```

## 4. Check Available Balance

```bash
stellar contract invoke --id CCHXTE7GP2M6RPJACBH7O6YHOX5O24VNTAYHZV3UMZBMD7BRGYCILHGO --source-account eren --network testnet -- get_available_balance --campaign_id 0
```

## 5. Check User Donation

```bash
stellar contract invoke --id CCHXTE7GP2M6RPJACBH7O6YHOX5O24VNTAYHZV3UMZBMD7BRGYCILHGO --source-account eren --network testnet -- get_donated_by_user --campaign_id 0 --user EREN_ADDRESS_HERE
```

## 6. Withdraw Funds

```bash
stellar contract invoke --id CCHXTE7GP2M6RPJACBH7O6YHOX5O24VNTAYHZV3UMZBMD7BRGYCILHGO --source-account eren --network testnet -- withdraw --campaign_id 0 --to EREN_ADDRESS_HERE --amount 5000000
```

## 7. Close Campaign

```bash
stellar contract invoke --id CCHXTE7GP2M6RPJACBH7O6YHOX5O24VNTAYHZV3UMZBMD7BRGYCILHGO --source-account eren --network testnet -- close_campaign --campaign_id 0
```

## 8. Check Campaign After Closing

```bash
stellar contract invoke --id CCHXTE7GP2M6RPJACBH7O6YHOX5O24VNTAYHZV3UMZBMD7BRGYCILHGO --source-account eren --network testnet -- get_campaign --campaign_id 0
```

The campaign status should become:

```txt
active: false
```

## Example Flow

```txt
1. Build the smart contract
2. Deploy the contract to Stellar Testnet
3. Create a donation campaign
4. Donate tokens to the campaign
5. Check the total donation
6. Withdraw funds as the owner
7. Close the campaign
```

## Expected Result

After running the full flow:

```txt
Campaign is created successfully
Donation is recorded
Total donated amount increases
Owner can withdraw funds
Campaign can be closed
```

## Author

Name: d d  
Workshop: Rise In x Stellar  
Project: Stellar Donation Box
