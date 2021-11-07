# Substrate Chaoscope

RPC Instrumentation toolkit for [pallet-chaos](https://github.com/paritytech/pallet-chaos).

Built with [subxt](https://github.com/paritytech/subxt).

# Introduction

In the Ethereum world, Smart Contracts are audited for their correctness and security. The audit process starts from the determinism of the EVM, and a detailed analysis of most (ideally all) possible logical scenarios that the contract bytecode can generate is carried out with the goal of asserting that:
- The Smart Contract does what it claims to do (correctness).
- The Smart Contract does not do what it never claimed to do (transparency).
- The Smart Contract cannot be exploited via attack vectors (security).

The majority of Ethereum Smart Contracts are written as Solidity/Vyper source code. Like any programming language, reusable logic is an emerging phenomenon. This gives rise to the OpenZeppeling business model, where pre-audited reusable Solidity code is provided as Open Source Software, accelerating development and increasing security across the whole Ethereum Ecosystem.

In terms of Substrate, a similar analysis can be done, although in two separate categories:
- **Runtime Pallets**: the State Transition Function (STF) of each Runtime consists of a set of deterministic input/output logical operations, commonly modularized into pallets. The audit process of a pallet consists of a detailed analysis of the correctness and security of most (ideally all) possible logical scenarios that a Runtime can be subjected to when using such a pallet.
- **Smart Contracts**: also consist of a set of deterministic input/output logic, but instead of having it reside as Runtime extrinsics, such logic is encoded as bytecode under some custom Virtual Machine specification (usually Turing Complete). The audit process of a Substrate Smart Contract starts at the VM specification and checks for the correctness and security of most (if not all) possible logical scenarios that the Smart Contract bytecode can be subjected to. Note: Substrate Smart Contracts are not covered in the context of this report.

In the Substrate Ecosystem, Audit stakeholders can be listed as follows:
- **Auditing Firm**: a third party service provider. Ideally, they have a solid grasp on the audited technologies and a know-how on offensive security best practices. Ex.: CertiK, SRLabs.
- **Audited Team**: some Substrate team who wrote a Runtime (or a pallet) and needs their source code audited. Ex.: Acala.
- **Substrate Ecosystem**: any teams that benefit from the Open Source Software written by the Audited Team. Ex.: some SBP team using a pre-audited pallet.
- **Ecosystem Curator**: teams working for the quality, success and security of the Ecosystem. Ex.: Parity, W3F.
- **User Base**: most users don’t have the time to verify 100% of the technology they’re using, so audited technologies provide trust to the user base. Ex.: Acala DeFi traders.

# Runtime Attack Surfaces

## Economic Vectors 

Substrate Runtimes can be seen as Distributed Virtual Machines (DVM), even when they’re not necessarily executing Smart Contracts. The difference is that Smart Contract-free Runtimes represent DVMs with static logic, and not really programmable via Smart Contracts bytecode. Such DVMs can however go through Runtime Upgrades, essentially upgrading DVM OpCodes. 

Whenever the DVM needs to execute its State Transition Functions, there’s a specific cost attributed to the virtual computational resources allocated for such transformation. Such cost is covered in the form of fees charged to the user, so that the system can function in a sustainable manner.

The relationship between fees and computational resources allocated from the DVM is critical to the economical safety of the system. Poorly designed fees leave room for wide Denial of Service (DoS) attack surfaces. An attacker can flood the system with resource-expensive but economically-cheap operations, effectively denying service from a significant number of users, while only spending a small amount of tokens.

While Ethereum introduced the concept of Gas as a correlation between EVM resource consumption and economical fees, Substrate uses Weights so that Runtime Engineers can attribute appropriate fees for each extrinsic operation of their pallets. Properly engineered transaction fees are a fundamental property of a attack-resistant pallet.

## Protocol Vectors

// ToDo: describe protocol attack vectors.

# Runtime Instrumentation

Chaoscope gives insight about the `pallet-chaos` extrinsics effects on Runtimes that include it.

Below are some empirical observations collected from a `substrate-node-template` with `pallet-chaos` added to it.
The machine had the following specs:

- CPU:
- RAM:
- Disk:
- PCI Drivers:
- Other Hardware:

For each extrinsic provided by `pallet-chaos`, a table is displayed with the relevant data.

## dragBlockUnitWeight(n)

Drags block production by calculating hashes in a loop (`n` times), with constant unitary extrinsic weight.

- `W = k + A * n`
- `drag_block_W = 1`

|                
|       n       | added block time |
|:-------------:|:----------------:|
|   1_000_000   |         x        |
|   5_000_000   |         x        |
|   10_000_000  |         x        |
|   50_000_000  |         x        |
|  100_000_000  |         x        |
|  500_000_000  |         x        |
| 1_000_000_000 |         x        |

## dragBlockDampWeight(wd, n)

- `W = k + A * n`
- `drag_block_W = k + wd * A * n`

Drags block production by calculating hashes in a loop (`n` times), with linear damping on weight (`0.0 < wd < 1.0`).

|       n       |  wd | added block time |
|:-------------:|:---:|:----------------:|
|   1_000_000   | 0.3 |         x        |
|   5_000_000   | 0.3 |         x        |
|   10_000_000  | 0.3 |         x        |
|   50_000_000  | 0.3 |         x        |
|  100_000_000  | 0.3 |         x        |
|  500_000_000  | 0.3 |         x        |
| 1_000_000_000 | 0.3 |         x        |
|   1_000_000   | 0.6 |         x        |
|   5_000_000   | 0.6 |         x        |
|   10_000_000  | 0.6 |         x        |
|   50_000_000  | 0.6 |         x        |
|  100_000_000  | 0.6 |         x        |
|  500_000_000  | 0.6 |         x        |
| 1_000_000_000 | 0.6 |         x        |
|   1_000_000   | 0.9 |         x        |
|   5_000_000   | 0.9 |         x        |
|   10_000_000  | 0.9 |         x        |
|   50_000_000  | 0.9 |         x        |
|  100_000_000  | 0.9 |         x        |
|  500_000_000  | 0.9 |         x        |
| 1_000_000_000 | 0.9 |         x        |

# License

<sup>
The entire code within this repository is licensed under the <a href="LICENSE">GPLv3</a>.
Please <a href="https://www.parity.io/contact/">contact us</a> if you have questions about the licensing of our
 products.
</sup>