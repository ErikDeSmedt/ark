# Compile from source

For the time being, we do not release pre-compiled binaries.
You can compile `bark` and `aspd` using the following instructions

## Install the Rust toolchain

The binaries are written in the Rust programming language. If you are running
Linux you can install Rust by executing the following command in your terminal.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If you are not using Linux you can find installation instructions on the [rustup](https://rustup.rs)-website.

## Cloning the source

The source is available in a [github](https://github.com/ark-bitcoin/ark)-repository that
contains source-code for `aspd` and `bark`.

To clone the repository run

```bash
git clone https://github.com/ark-bitcoin/ark
cd ark
```

## Compile the binaries

You can compile the binaries using `cargo`. Ensure you are in the `ark`-folder
and execute

```bash
cargo build --workspace
```

Once the command is completed the executables will appear in your target directory.

You can verify that the executables work by running

```bash
./target/debug/bark --version
./target/debug/aspd --version
```

The `bark` and `aspd` program are single-file executables. You can move them to any place
on your file-system and they will work.

In this tutorial we will assume that `bark` and `aspd` are located on your path.

Run the following command in your terminal to modify the path of your current session.

```bash
export PATH=$PATH:$PWD/target/debug/
```

If you want to ensure that `bark` and `aspd` are permanently on your path.
You can consider moving them to the `/usr/local/bin`-directory which
is a good choice for most Linux distro's.

```bash
mv ./target/debug/bark /usr/local/bin
mv ./target/debug/aspd /usr/local/bin
```

Once the executable is on your path you can execute.

```bash
bark --version
aspd --version
```

# Additional dependencies

We do recommend to install the following dependencies.

- [Bitcoin Core](https://bitcoincore.org): To run `aspd`, follow the tests or create a demo on `regtest`.
- [just](https://github.com/casey/just): To run the tests
- [jq](https://github.com/casey/just): To run the tests and follow tutorial.

## Install Bitcoin Core

You need Bitcoin Core if you want to run `aspd`. This is strongly recommended.

You can download the prebuilt binaries from the [Download page](https://www.bitcoincore.org/en/download).
For Linux systems you can unpack the zip and add `bitcoind` and `bitcoin-cli` to your path.

## Install just

Just is a command runner that saves you some key-strokes.
You need `just` to run the tests.

You can install it using `cargo`

```bash
cargo install just
```

## Install jq

This is command-line tool that can query and transform `json`-data. It is a dependency
for to run the tests and follow our demo-scenario.

It is widely used and is available on most package managers such as `apt` or `dnf`.
You can install it using one of the options below.

If you are using Ubuntu or another debian based distro

```bash
sudo apt install jq
```

If you are using Fedora, CentOS or Rocky Linux

```bash
dnf install jq
```

or download the binarie from the [github releases page](https://github.com/jqlang/jq/releases).


# Running the tests (optional)

Ensure you have installed all of the dependencies mentioned above. You can execute

- `just test-unit` to perform all unit-tests
- `just test-integration` to perform all integration tests
- `just test` to perform all tests
