# fair-baccarat

CLI tool and library that verifies provably fair bets for baccarat game. Compatible with Stake.com [provably fair algorithm](https://stake.com/casino/games/baccarat?clientSeed=client%20seed&game=baccarat&modal=verify&nonce=2&serverSeed=server%20seed).

[![Build Status](https://travis-ci.org/lucasholder/fair-baccarat.svg?branch=master)](https://travis-ci.org/lucasholder/fair-baccarat)
[![crates.io](https://meritbadge.herokuapp.com/fair-baccarat)](https://crates.io/crates/fair-baccarat)

## Install

On Mac or Linux:

```bash
curl -sL
https://raw.githubusercontent.com/lucasholder/fair-baccarat/master/install.sh | sh
```

If you have Rust:

```bash
cargo install fair-baccarat
```

## Usage

```bash
fair-baccarat <client_seed> <server_seed> <nonce>
```

Example usage:

```bash
$ fair-baccarat "client seed" "server seed" 2
Client seed: client seed
Server seed: server seed
Nonce: 2

Player won

Player (9): ♦9 - ♦10
Banker (7): ♥4 - ♦3
```

As expected, we get the same result as on
[Stake.com](https://stake.com/casino/games/baccarat?clientSeed=client%20seed&game=baccarat&modal=verify&nonce=2&serverSeed=server%20seed).

## Rust API docs

[fair_baccarat](https://docs.rs/fair-baccarat/)
