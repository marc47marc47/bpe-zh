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

---

## Code Analysis (`src/main.rs`)

The code is primarily composed of the following parts:

1.  **`Token` struct**: Represents a token and tracks if it is a whitespace character.
2.  **`split_chars`**: Initializes the process by splitting the input text into a sequence of single-character `Token`s.
3.  **`count_bigrams`**: Counts the frequency of all adjacent (non-whitespace) token pairs in the sequence.
4.  **`most_frequent_bigram`**: Finds the most common token pair from the frequency map.
5.  **`merge_tokens`**: Merges a specified token pair into a new, single token.
6.  **`run_bpe`**: The main loop for the BPE algorithm. It iteratively performs the "count" and "merge" steps until a stopping condition is met.
7.  **`main` function**: The program's entry point, responsible for parsing command-line arguments and calling `run_bpe`.
8.  **`tests` module**: Contains unit tests to ensure the correctness of individual functions.

---

## Development Journey and Analysis

The project's evolution reflects several common stages and challenges in command-line tool design.

### Feature Evolution

1.  **Initial Version**: The logic and input text were hardcoded in the `main` function, serving only as a functional demo.
2.  **Parameterization V1**: Introduced `clap` to add `--file`, `--text`, and `--max-iterations` arguments, making the tool more generic.
3.  **Parameterization V2**: Based on user feedback, the stopping condition was changed from a fixed number of iterations to the more intuitive "token compression percentage" (`--percentage`).
4.  **Final Architecture (Subcommands)**: To cleanly support both "percentage" and the new "frequency" modes, the project was refactored to use `clap`'s subcommand architecture. This is a best practice for handling multiple, distinct modes of operation.

### Development and Debugging Analysis

During the final refactoring, several noteworthy technical challenges were encountered:

*   **`clap` Parameter Validation**: An incorrect syntax for `clap` v4's `range` validator on the `value_parser` for the `frequency` mode's `-n` argument caused several compilation failures. The final solution was to remove the validation from the argument declaration and instead perform a manual `if` check in the `main` function after parsing. This approach, while less "elegant," is more direct, stable, and bypasses potential syntax issues with specific `clap` versions.

*   **`println!` Formatting String Error**: In one of the fixes, an unrecognizable non-ASCII character was accidentally introduced into a `println!` format string, leading to an `invalid format string` compilation error. This simple mistake serves as a reminder to be extra cautious during copy-pasting or rapid code editing, especially with strings.

Through these iterations and fixes, the code reached a stable, well-structured, and powerful state.


### Run example
```bash
$ sh test-sample01.sh
    Finished `release` profile [optimized] target(s) in 0.06s
     Running `target/release/bpe-zh frequency -f sample01.txt -n 2`
🚀 **BPE 演算法開始**
初始 Token 數量: 114
模式: Frequency | 目標: 最高頻率 <= 2
═══════════════════════════════════════
**Step 1**: 最常見組合 `奶 + 奶` 出現 **16** 次
  ├─ 合併後 Token 數量: 98
**Step 2**: 最常見組合 `牛 + 奶奶` 出現 **7** 次
  ├─ 合併後 Token 數量: 91
**Step 3**: 最常見組合 `牛 + 奶` 出現 **7** 次
  ├─ 合併後 Token 數量: 84
**Step 4**: 最常見組合 `柳 + 奶奶` 出現 **5** 次
  ├─ 合併後 Token 數量: 79
**Step 5**: 最常見組合 `的 + 牛奶` 出現 **4** 次
  ├─ 合併後 Token 數量: 75
**Step 6**: 最常見組合 `劉 + 奶奶` 出現 **4** 次
  ├─ 合併後 Token 數量: 71
✅ **目標達成**: 最高頻率 (2) 已滿足目標 (<= 2).
═══════════════════════════════════════
🎯 **最終結果**: [劉奶奶, 找, 牛奶奶, 買, 牛奶, ，, ⎵, 牛奶奶, 給, 劉奶奶, 拿, 牛奶, ，, ⎵, 劉奶奶, 說, 牛奶奶, 的牛奶, 不, 如, 柳奶奶, 的牛奶, ，, ⎵, 牛奶奶, 說, 柳奶奶, 的牛奶, 會, 流, 奶, ，, ⎵, 柳奶奶, 聽, 見, 了, 大, 罵, 牛奶奶, 你, 的, 才, 會, 流, 奶, ，, ⎵, 柳奶奶, 和, 牛奶奶, 潑, 牛奶, 嚇, 壞, 了, 劉奶奶, ，, ⎵, 大, 罵, 再, 也, 不, 買, 柳奶奶, 和, 牛奶奶, 的牛奶, 。, ⎵]
Token 數量: 114 → 71

```
