# Token Vault Program

A Solana program that implements a secure vault system for managing SOL tokens with deposit and withdrawal functionality.
It is being used for fee management on tokenflowzy token creator!

## Overview

This program provides a secure way to store and manage SOL tokens on the Solana blockchain. It features:

- Vault initialization with owner authentication
- Secure token deposits
- Owner-restricted withdrawals
- Event emission for tracking transactions
- Built using the Anchor framework

## Program Architecture

The program consists of three main instructions:

1. `initialize` - Creates a new vault with a designated owner
2. `deposit` - Allows any user to deposit SOL into the vault
3. `withdraw` - Enables the owner to withdraw funds from the vault

## Account Structure

- `VaultState`: Main account that stores:
  - Owner's public key
  - Vault state bump seed
  - Total revenue
  - Number of tokens deployed

## Installation

1. Install dependencies:
```sh
npm install