
##  Project Overview

A Decentralized Task Management System where users can:

* Create and manage personal task lists
* Assign tasks to other users
* Reward task completion with tokens
* Track task history and statistics

This project demonstrates Solana core concepts: accounts, PDAs, CPIs, rent-exemption, transactions, and token handling.

---

## Architecture Overview

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   User Wallet   │────│  Task Manager    │────│  Reward Token   │
│   (Account)     │    │    Program       │    │    Program      │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                                │
                       ┌────────┴────────┐
                       │                 │
                  ┌────▼────┐      ┌────▼────┐
                  │  Task   │      │ User    │
                  │ Account │      │Profile  │
                  │  (PDA)  │      │ (PDA)   │
                  └─────────┘      └─────────┘
```

---

##  Core Concepts Implementation 

1. **Solana Account Model**

   * User Profile Account: Stores user stats, completed tasks count
   * Task Account: title, description, status, assignee
   * Token Account: SPL token accounts for rewards
   * Program Account: Smart contract (Anchor recommended)

2. **Program Derived Address (PDA)**

   * User profile PDA: `Pubkey::find_program_address(&[b"user_profile", user_pubkey.as_ref()], program_id)`
   * Task PDA: `Pubkey::find_program_address(&[b"task", user_pubkey.as_ref(), &task_id.to_le_bytes()], program_id)`
   * Global stats: `[b"global_stats"]`

3. **Transactions & Instructions**

   * `initialize_user`
   * `create_task`
   * `assign_task`
   * `complete_task`
   * `claim_reward`

4. **Cross Program Invocation (CPI)**

   * Use SPL Token program to mint or transfer reward tokens
   * Use System program to create rent-exempt accounts

5. **Transaction Fees**

   * Demonstrate rent-exemption and `getMinimumBalanceForRentExemption`
   * Show fee estimation and examples

---

##  Stack

* Program: **Anchor (Rust)**
* Client: **TypeScript** with `@project-serum/anchor` and `@solana/web3.js`
* Token: `@solana/spl-token` for client-side token interactions

---

