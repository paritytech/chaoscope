# Substrate Chaoscope

**Substrate Chaoscope** is a [subxt](https://github.com/paritytech/subxt) based RPC Instrumentation toolkit for [pallet-chaos](https://github.com/paritytech/pallet-chaos).


Chaoscope makes Substrate Runtimes behave in ways that they're not supposed to, with the following goals: 
- Explore Runtime edge cases. 
- Explore Extrinsic weights and their economic implications.
- Expose Runtime Attack Vectors.
- Raise awareness about Benchmarking and Weight Design.
- 
# Similar Projects
- [PolPatrol – Validator for Polkadot Runtimes](https://chainsecurity.com/polpatrol-validator-for-polkadot-runtimes/)

# Runtime Attack Surfaces

## Economic Vectors

Substrate Runtimes can be seen as Distributed Virtual Machines (DVM), even when they’re not necessarily executing Smart Contracts. The difference is that Smart Contract-free Runtimes represent DVMs with static logic, and not really programmable via Smart Contracts bytecode. Such DVMs can however go through Runtime Upgrades, essentially upgrading DVM OpCodes.

Whenever the DVM needs to execute its State Transition Functions, there’s a specific cost attributed to the virtual computational resources allocated for such transformation. Such cost is covered in the form of fees charged to the user, so that the system can function in a sustainable manner.

The relationship between fees and computational resources allocated from the DVM is critical to the economical safety of the system. Poorly designed fees leave room for wide Denial of Service (DoS) attack surfaces. An attacker can flood the system with resource-expensive but economically-cheap operations, effectively denying service from a significant number of users, while only spending a small amount of tokens.

While Ethereum introduced the concept of Gas as a correlation between EVM resource consumption and economical fees, Substrate uses Weights so that Runtime Engineers can attribute appropriate fees for each extrinsic operation of their pallets. Properly engineered transaction fees are a fundamental property of a attack-resistant pallet.

# Instructions

1. To deploy a test setup:
```sh
$ git clone https://github.com/paritytech/chaoscope.git
$ cd chaoscope
$ bash chaoscope.sh
```

The script will deploy a `substrate-node-template` with `pallet-chaos`, fire extrinsics to it and report the resutls.

# Roadmap

- [x] `drag_block_unit_weight` extrinsic implementation
- [x] `drag_block_unit_weight` subxt RPC
- [x] `substrate-node-chaos` (`substrate-node-chaos`+`pallet-chaos`)
- [x] `chaoscope.sh`
- [ ] Relay Chain support on `chaoscope.sh` (`polkadot`+[`cumulus`+`pallet-chaos`])

# License

<sup>
The entire code within this repository is licensed under the <a href="LICENSE">AGPLv3</a>.
Please <a href="https://www.parity.io/contact/">contact us</a> if you have questions about the licensing of our
 products.
</sup>
