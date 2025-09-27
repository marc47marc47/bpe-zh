# BPE-ZH: A BPE Tokenizer

This is a command-line tool written in Rust that performs Byte-Pair Encoding (BPE) to tokenize text. It is designed to be flexible, allowing users to control the tokenization process through different stopping conditions.

## Core Features

- **BPE Algorithm**: Implements the core Byte-Pair Encoding logic to merge frequent character pairs iteratively.
- **Flexible Input**: Accepts text input from either a file (`--file`) or a direct command-line string (`--text`).
- **Subcommand-based Interface**: Uses a clear subcommand structure to switch between different modes of operation.
- **Two Stopping-Condition Modes**:
  - `percentage` mode: Stops merging when the total token count is reduced to a target percentage of the original count.
  - `frequency` mode: Stops merging when the count of the most frequent character pair drops to or below a specified threshold.
- **Packaging Script**: Includes a `pack-bin.sh` helper script to compile and package the application binary for easy distribution.

---

## How to Use

The tool uses subcommands (`percentage` or `frequency`) to determine its behavior. The basic syntax is:

```sh
./bpe-zh [INPUT_OPTIONS] <COMMAND> [COMMAND_OPTIONS]
```

#### 1. Build the Project

First, build the project in release mode for optimal performance:
```bash
cargo build --release
```
The executable will be available at `./target/release/bpe-zh`.

#### 2. Input Options (Global)

You must provide an input source using one of the following global options:

| Short | Long   | Value         | Description                  |
| :---- | :----- | :------------ | :--------------------------- |
| `-f`  | `--file` | `<FILE_PATH>` | Read input from a text file. |
| `-t`  | `--text` | `<TEXT>`      | Read input from a CLI string.|


#### 3. Commands

You must choose one of the following commands to execute:

##### `percentage`

This mode stops the BPE process once the token count has been compressed to a certain percentage of the initial count. This is the most intuitive way to control the overall compression rate.

**Options:**
| Short | Long         | Value     | Description                                    |
| :---- | :----------- | :-------- | :--------------------------------------------- |
| `-p`  | `--percentage` | `<1-100>` | **Target Compression Rate**. Defaults to `70`. |


##### `frequency`

This mode stops the BPE process when the highest frequency of any character pair is less than or equal to a given threshold. This is useful for creating a vocabulary containing only high-frequency pairs.

**Options:**
| Short | Long        | Value   | Description                                  |
| :---- | :---------- | :------ | :------------------------------------------- |
| `-n`  | `--frequency` | `>=2`    | **Frequency Threshold**. Stops when `max_freq <= n`. |

---

## Examples

#### Example 1: Using `percentage` mode to compress to 50%

```bash
./target/release/bpe-zh --text "Learning new things is always exciting." percentage -p 50
```

#### Example 2: Using `frequency` mode until max frequency is 2 or less

```bash
./target/release/bpe-zh -t "a a a a b b b c c d" frequency -n 2
```
*In this example, `a a` will be merged first, followed by `b b`. The algorithm will then stop because the next most frequent pair, `c c`, has a frequency of 2, which meets the stopping condition `max_freq <= 2`.*

#### Example 3: Using file input with the `percentage` mode

```bash
# 1. Create an input file
echo "The quick brown fox jumps over the lazy dog." > input.txt

# 2. Run the program
./target/release/bpe-zh -f input.txt percentage -p 60
```

#### Example 4: Getting help for a specific subcommand

```bash
./target/release/bpe-zh percentage --help
```

---

## Packaging for Distribution

The project includes a shell script to simplify packaging.

```bash
# 1. Make the script executable (only needs to be done once)
chmod +x pack-bin.sh

# 2. Run the script
./pack-bin.sh
```

This will create a compressed archive (`.tar.gz`) in the root directory, named according to the pattern: `bpe-zh-v<VERSION>-<OS>-<ARCH>.tar.gz`.
