# formal-verso: Formal Verification for Soroban

The Formal VerSo project aims to create formal verification tools for Soroban,
ensuring smart contract correctness and safety. We envision a future where users
can easily define Soroban application behavior and verify it in an automated
manner. Formal VerSo utilizes [Galois’s Software Analysis
Workbench](https://saw.galois.com/), a tool for formally verifying code in
languages like Rust, C, Java, or Cryptol. SAW employs SAT and SMT solvers to
streamline verification, offering SAWScript for scaling up verification to
complex systems.

A recording of the PoC demo can be found [here](https://youtu.be/cwaXmZwbXsA?feature=shared) (YT link).

**Note: this project is still in its early stages, and only supports a small
fraction of the Soroban API. In addition, due to tooling limitations, we
currently only support v0.8.4 of the Soroban SDK. We are actively working to
support more APIs and versions.**

This repository contains:
- `lib`: A SAWScript+Cryptol library for verifying Soroban smart contracts
- `soroban-examples` (submodule): A fork of the [official `soroban-examples`
  repo](https://github.com/stellar/soroban-examples) at v0.8.4, with slight
  configuration changes to enable symbolic execution of contracts
- `extra-examples`: More examples of contracts that are suitable for formal
  verification
- `example-proofs`: SAWScript proofs of correctness for selected examples from
  `soroban-examples` and `extra-examples`
  - `increment.saw`: Verifies `soroban-examples/increment`. Demonstrates
    verification involving the Storage and Short Symbol APIs.
  - `alloc.saw`: Verifies `soroban-examples/alloc`. Demonstrates verification
    involving dynamic allocation.
  - `sqrt.saw`: Verifies `extra-examples/sqrt`. Demonstrates verification
    involving complex computations, the Storage API, and compositional
    verification.

## Installation

1. Install SAW
   1. Clone [`saw-script`](https://github.com/GaloisInc/saw-script)
   2. Switch to branch `soroban`
   3. [Build from
      source](https://github.com/GaloisInc/saw-script#manual-installation)
   4. `cabal v2-install`
2. [Install
   `mir-json`](https://github.com/GaloisInc/mir-json#installation-instructions)
3. Install `crux-mir` Rust libraries
   1. Clone [`crucible`](https://github.com/GaloisInc/crucible)
   2. `cd crux-mir`
   3. `TARGET=wasm32-unknown-unknown ./translate_libs.sh`
   4. Add `export SAW_RUST_LIBRARY_PATH="/path/to/crucible/crux-mir/rlibs"` to
      your `.bashrc` or similar

## Running the examples

1. Clone this repo
2. `git submodule update --init`
3. For each contract that you want to verify:
   1. `cd` to the contract package
   2. `make saw-build`
4. `cd example-proofs`
5. `saw <example-proof>.saw`
   1. Note: `sqrt` takes a long time (hours) to run from scratch. To use cached
      SMT solver results, run `SAW_SOLVER_CACHE_PATH=solver-cache saw sqrt.saw`.
