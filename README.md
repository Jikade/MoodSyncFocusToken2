# MoodSync Focus Token

## Problem

Students often lose motivation during self-study because focus time, streaks, and learning effort are easy to fake, edit, or forget in normal apps.

## Solution

MoodSync Focus Token is a Stellar/Soroban smart contract that rewards users with FOCUS points when they complete verified focus sessions.

## Why Stellar

Stellar makes the reward system more transparent and tamper-resistant by storing focus rewards, streaks, and redemption actions on-chain through Soroban smart contracts.

## Target User

MoodSync Focus Token is built for students, self-learners, and study communities who want a more trustworthy and motivating way to track focus time and learning consistency.

## Live Demo

* Network: Stellar Testnet

* **Contract ID**: `CACRYT6VQFC5P7BCGXPBQKA57GJX7VXHE6XUJY2KECHUD2ZV6OK2ZOED`

* **Transaction**: 73752fe3ead02a08f5da4462772b15c49e1de06d81abacc39162b636128cae1f

Example Stellar Expert format:

```text
https://stellar.expert/explorer/testnet/contract/CACRYT6VQFC5P7BCGXPBQKA57GJX7VXHE6XUJY2KECHUD2ZV6OK2ZOED
```

## Core Features

* Start a focus session
* Complete a focus session
* Calculate FOCUS reward based on study duration and AI score
* Track user FOCUS balance
* Track user streak
* Redeem FOCUS for in-app rewards
* Award demo FOCUS tokens for testing

## Smart Contract Functions

| Function                                       | Description                                |
| ---------------------------------------------- | ------------------------------------------ |
| `start_session(user, session_id, minutes)`     | Creates a new focus session for a user     |
| `complete_session(user, session_id, ai_score)` | Completes a session and rewards FOCUS      |
| `balance(user)`                                | Returns the user's current FOCUS balance   |
| `streak(user)`                                 | Returns the user's current learning streak |
| `redeem(user, item, cost)`                     | Redeems FOCUS for an in-app item           |
| `award(user, amount)`                          | Adds FOCUS to a user for demo/testing      |

## Reward Rules

| Condition                       |     Reward |
| ------------------------------- | ---------: |
| Every 25 minutes of focus       | `+1 FOCUS` |
| AI score is 80 or higher        | `+1 FOCUS` |
| Session is 50 minutes or longer | `+1 FOCUS` |

Example:

```text
25-minute session + AI score 85 = 2 FOCUS
```

## How to Run

### 1. Open Project in Soroban Studio

This project was built on:

```text
https://soroban.studio/
```

Open the project and edit:

```text
contracts/hello_world/src/lib.rs
```

### 2. Build Contract

```bash
stellar contract build
```

### 3. Deploy to Stellar Testnet

```bash
stellar contract deploy --wasm target/wasm32v1-none/release/hello_world.wasm --source khoa --network testnet
```

After deployment, copy the new Contract ID.

### 4. Check Contract Functions

```bash
stellar contract invoke --id CACRYT6VQFC5P7BCGXPBQKA57GJX7VXHE6XUJY2KECHUD2ZV6OK2ZOED --source khoa --network testnet -- --help
```

Expected functions:

```text
award
redeem
streak
balance
start_session
complete_session
```

### 5. Fund Testnet Account

If the account is not funded yet:

```bash
stellar keys fund khoa --network testnet
```

### 6. Test Balance

```bash
stellar contract invoke --id CACRYT6VQFC5P7BCGXPBQKA57GJX7VXHE6XUJY2KECHUD2ZV6OK2ZOED --source khoa --network testnet -- balance --user user1
```

Expected result:

```text
"0"
```

### 7. Start Focus Session

```bash
stellar contract invoke --id CACRYT6VQFC5P7BCGXPBQKA57GJX7VXHE6XUJY2KECHUD2ZV6OK2ZOED --source khoa --network testnet --send=yes -- start_session --user user1 --session_id 1 --minutes 25
```

### 8. Complete Focus Session

```bash
stellar contract invoke --id CACRYT6VQFC5P7BCGXPBQKA57GJX7VXHE6XUJY2KECHUD2ZV6OK2ZOED --source khoa --network testnet --send=yes -- complete_session --user user1 --session_id 1 --ai_score 85
```

Expected result:

```text
"2"
```

### 9. Check FOCUS Balance

```bash
stellar contract invoke --id CACRYT6VQFC5P7BCGXPBQKA57GJX7VXHE6XUJY2KECHUD2ZV6OK2ZOED --source khoa --network testnet -- balance --user user1
```

Expected result:

```text
"2"
```

### 10. Check Streak

```bash
stellar contract invoke --id CACRYT6VQFC5P7BCGXPBQKA57GJX7VXHE6XUJY2KECHUD2ZV6OK2ZOED --source khoa --network testnet -- streak --user user1
```

Expected result:

```text
"1"
```

### 11. Redeem Reward

```bash
stellar contract invoke --id CACRYT6VQFC5P7BCGXPBQKA57GJX7VXHE6XUJY2KECHUD2ZV6OK2ZOED --source khoa --network testnet --send=yes -- redeem --user user1 --item blue_theme --cost 1
```

### 12. Award Demo Token

```bash
stellar contract invoke --id CACRYT6VQFC5P7BCGXPBQKA57GJX7VXHE6XUJY2KECHUD2ZV6OK2ZOED --source khoa --network testnet --send=yes -- award --user user1 --amount 10
```

## Test Cases

| Test Case | Command / Action                        | Expected Result                                                                            |
| --------- | --------------------------------------- | ------------------------------------------------------------------------------------------ |
| TC01      | Check contract help                     | Contract shows `award`, `redeem`, `streak`, `balance`, `start_session`, `complete_session` |
| TC02      | Check balance of new user               | Returns `"0"`                                                                              |
| TC03      | Start 25-minute session                 | Session is created successfully                                                            |
| TC04      | Complete session with AI score 85       | Returns `"2"`                                                                              |
| TC05      | Check balance after complete session    | Balance increases by `2`                                                                   |
| TC06      | Check streak after complete session     | Streak increases by `1`                                                                    |
| TC07      | Redeem item with enough balance         | Balance decreases by item cost                                                             |
| TC08      | Redeem item with insufficient balance   | Contract returns an error                                                                  |
| TC09      | Start session under 25 minutes          | Contract returns an error                                                                  |
| TC10      | Complete session with AI score over 100 | Contract returns an error                                                                  |

## Tech Stack

* Smart Contract: Rust / Soroban SDK
* Blockchain Network: Stellar Testnet
* Development Environment: Soroban Studio
* CLI: Stellar CLI
* Storage: Soroban Persistent Storage
* Contract Output: WebAssembly/Wasm

## Project Structure

```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

## Future Improvements

* Connect with a frontend dashboard
* Add Freighter wallet login
* Replace text user IDs with Stellar wallet addresses
* Add leaderboard by FOCUS balance
* Add daily streak logic based on timestamps
* Add NFT badge rewards for long learning streaks
* Integrate MoodSync AI score from the main app backend

## Team

* Nguyễn Tất Khoa | @nguyentatkhoa2005 | [nguyentatkhoa2005@gmail.com](mailto:nguyentatkhoa2005@gmail.com) | HUTECH - Year 2
