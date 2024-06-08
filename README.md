# Verifiers for Hylé

Welcome to the Verifiers for Hylé repository! This repository is dedicated to hosting all the different verifiers that Hylé supports. Each verifier is organized into its own folder, making it straightforward to add new verifiers and maintain existing ones.

## Project Description

The **Verifiers for Hylé** repository is a comprehensive collection of verifiers supported by Hylé. Verifiers play a crucial role in validating and verifying various operations within the Hylé ecosystem. This repository is structured to facilitate easy addition and maintenance of verifiers, ensuring a robust and scalable verification system.

### Code Structure

The repository adopts a modular approach, with each verifier housed in its own folder within the `verifiers` directory. This structure enhances clarity and ease of navigation, making it simple to manage and expand the repository. Below is an overview of the directory structure:

```text
verifiers-for-hyle
├── Cargo.toml
├── verifiers
│   ├── verifier1
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs                    <-- [Verifier1 code goes here]
│   ├── verifier2
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs                    <-- [Verifier2 code goes here]
│   └── ...                                <-- [Additional verifiers]
└── README.md
```

To add a new verifier, simply create a new folder within the `verifiers` directory and follow the structure of the existing verifiers.

## Quick Start

Before you begin, ensure that [rustup] is installed. The [`rust-toolchain.toml`][rust-toolchain] file will be used by `cargo` to automatically install the correct version.

To build all methods and execute the method within the zkVM, use the following command:

```bash
cargo run
```

Since this is an initial template, no output is expected until the code is modified.

### Executing the Project Locally in Development Mode

For faster iterations during development, we recommend leveraging [dev-mode]. This mode accelerates the development process and provides execution statistics. To run your project in development mode with execution statistics, use the following command:

```bash
RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run
```

### Running Proofs Remotely on Bonsai

_Note: The Bonsai proving service is in early Alpha. An API key is required for access. [Click here to request access][bonsai access]._

If you have access to the Bonsai URL and API key, you can run your proofs remotely. Use the following command, including the necessary environment variables:

```bash
BONSAI_API_KEY="YOUR_API_KEY" BONSAI_API_URL="BONSAI_URL" cargo run
```

## Creating a Project Based on This Template

Search this template for the string `TODO`, and make the necessary changes to implement the required features described by the `TODO` comments. For more complex changes, we provide several instructional resources:

- The [RISC Zero Developer Docs][dev-docs] is a great place to start.
- Example projects are available in the [examples folder][examples] of the [`risc0`][risc0-repo] repository.
- Reference documentation is available on [https://docs.rs][docs.rs], including [`risc0-zkvm`][risc0-zkvm], [`cargo-risczero`][cargo-risczero], [`risc0-build`][risc0-build], and [other crates][crates].

## Directory Structure

The following standard directory structure is used for zkVM applications in this template:

```text
project_name
├── Cargo.toml
├── host
│   ├── Cargo.toml
│   └── src
│       └── main.rs                        <-- [Host code goes here]
└── methods
    ├── Cargo.toml
    ├── build.rs
    ├── guest
    │   ├── Cargo.toml
    │   └── src
    │       └── bin
    │           └── method_name.rs         <-- [Guest code goes here]
    └── src
        └── lib.rs
```

## Video Tutorial

For a step-by-step guide on building with this template, watch this [excerpt from our workshop at ZK HACK III][zkhack-iii].

## Useful Links

- [Hylé website](https://www.hyle.org)
- [Hylé documentation](https://docs.hyle.org)
- [RISC Zero Developer Docs][dev-docs]
- Example projects in the [examples folder][examples] of the [`risc0`][risc0-repo] repository
- Reference documentation on [https://docs.rs][docs.rs] including [`risc0-zkvm`][risc0-zkvm], [`cargo-risczero`][cargo-risczero], and [`risc0-build`][risc0-build]

## Questions, Feedback, and Collaborations

We welcome your feedback and collaboration! Connect with us on [Discord][discord] or [Twitter][twitter].

[bonsai access]: https://bonsai.xyz/apply
[cargo-risczero]: https://docs.rs/cargo-risczero
[crates]: https://github.com/risc0/risc0/blob/main/README.md#rust-binaries
[dev-docs]: https://dev.risczero.com
[dev-mode]: https://dev.risczero.com/api/zkvm/dev-mode
[discord]: https://discord.gg/risczero
[docs.rs]: https://docs.rs/releases/search?query=risc0
[examples]: https://github.com/risc0/risc0/tree/main/examples
[risc0-build]: https://docs.rs/risc0-build
[risc0-repo]: https://www.github.com/risc0/risc0
[risc0-zkvm]: https://docs.rs/risc0-zkvm
[rustup]: https://rustup.rs
[rust-toolchain]: rust-toolchain.toml
[twitter]: https://twitter.com/risczero
[zkvm-overview]: https://dev.risczero.com/zkvm
[zkhack-iii]: https://www.youtube.com/watch?v=Yg_BGqj_6lg&list=PLcPzhUaCxlCgig7ofeARMPwQ8vbuD6hC5&index=5
