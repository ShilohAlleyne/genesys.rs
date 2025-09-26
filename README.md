# Genesys.rs

Genesys.rs is an simple cli for searching cards (and opening their YGO-Pro db entry) in the Genesys Yugioh format. Cards point values are automatically updated and changes in values between banlists are tracked. 

# Features

Search cards by:
- Point value
- Card type
- Archetype
- Level
- Name

# Installation

> Note: This CLI is simple, if its missing functionality either raise and issue or fork and add it yourself lol

<details>
<summary>From pre-built binaries</summary>

You can download pre-built binaries from the [releases page](https://github.com/ShilohAlleyne/genesys.rs/releases).
Just copy the binary to a directory in your `PATH` and make it executable.

</details>

<details>
<summary>From crates.io</summary>

You can install `genesys` using cargo:
```bash
cargo install genesys
```
This will install the latest version of `genesys` from [crates.io](https://crates.io/crates/genesys-ygo-cli).

</details>

<details>
<summary>From source</summary>

You can build `genesys` from source using cargo:

```bash
git clone https://github.com/ShilohAlleyne/genesys.rs
cd genesys
cargo install --path .
```

</details>
