use clap::{Args, Parser, Subcommand};
use std::collections::HashMap;
use std::fs;
use std::process;

// BPE 核心邏輯部分 (Token, split_chars, etc.)
// ... (這部分程式碼與之前相同，保持不變) ...

/// **字元分割結構 (Character Splitting Structure)**
#[derive(Debug, Clone)]
pub struct Token {
    content: String,
    is_space: bool,
}

impl Token {
    fn new(content: String) -> Self {
        let is_space = content.trim().is_empty();
        Self { content, is_space }
    }
}

/// **將文字分解成字元token (Split text into character tokens)**
fn split_chars(text: &str) -> Vec<Token> {
    text.chars().map(|c| Token::new(c.to_string())).collect()
}

/// **統計雙字組合頻率 (Count bigram frequencies)**
fn count_bigrams(tokens: &[Token]) -> HashMap<(String, String), usize> {
    let mut freq = HashMap::new();
    for window in tokens.windows(2) {
        if window[0].is_space || window[1].is_space {
            continue;
        }
        let pair = (window[0].content.clone(), window[1].content.clone());
        *freq.entry(pair).or_insert(0) += 1;
    }
    freq
}

/// **找到最頻繁的雙字組合 (Find most frequent bigram)**
fn most_frequent_bigram(freq: &HashMap<(String, String), usize>) -> Option<((String, String), usize)> {
    freq.iter()
        .max_by_key(|(_, count)| *count)
        .map(|(pair, count)| (pair.clone(), *count))
}

/// **合併指定的token組合 (Merge specified token pairs)**
fn merge_tokens(tokens: &[Token], target: &(String, String)) -> Vec<Token> {
    let mut result = Vec::with_capacity(tokens.len());
    let mut i = 0;
    while i < tokens.len() {
        if i < tokens.len() - 1
            && !tokens[i].is_space
            && !tokens[i + 1].is_space
            && tokens[i].content == target.0
            && tokens[i + 1].content == target.1
        {
            let merged_content = format!("{}{}", target.0, target.1);
            result.push(Token::new(merged_content));
            i += 2;
        } else {
            result.push(tokens[i].clone());
            i += 1;
        }
    }
    result
}

/// **格式化顯示tokens (Format tokens for display)**
fn format_tokens(tokens: &[Token]) -> String {
    tokens
        .iter()
        .map(|t| {
            if t.is_space {
                "⎵".to_string()
            } else {
                t.content.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// 定義演算法的停止條件
pub enum StoppingCondition {
    Percentage(u8),
    Frequency(usize),
}

/// **BPE演算法主要實現 (Main BPE algorithm implementation)**
pub fn run_bpe(text: &str, condition: StoppingCondition) -> Vec<Token> {
    let mut tokens = split_chars(text);
    let initial_token_count = tokens.len();
    if initial_token_count == 0 {
        return tokens;
    }

    let mut target_token_count = 0; // For percentage mode

    println!("🚀 **BPE 演算法開始**");
    println!("初始 Token 數量: {}", initial_token_count);

    match condition {
        StoppingCondition::Percentage(p) => {
            target_token_count = (initial_token_count as f64 * (p as f64 / 100.0)).ceil() as usize;
            println!("模式: Percentage | 目標: Token 數量 <= {} (原始數量的 {}%)", target_token_count, p);
        }
        StoppingCondition::Frequency(n) => {
            println!("模式: Frequency | 目標: 最高頻率 <= {}", n);
        }
    }
    println!("═══════════════════════════════════════");

    let mut step = 0;
    loop {
        if let StoppingCondition::Percentage(_) = condition {
            if tokens.len() <= target_token_count {
                println!("✅ **目標達成**: 目前 Token 數量 ({}) 已滿足目標 (<= {})。", tokens.len(), target_token_count);
                break;
            }
        }

        let freq = count_bigrams(&tokens);
        if freq.is_empty() {
            println!("🛑 **沒有更多可合併的 bigram，演算法結束**");
            break;
        }

        if let Some((bigram, count)) = most_frequent_bigram(&freq) {
            if let StoppingCondition::Frequency(n) = condition {
                if count <= n {
                    println!("✅ **目標達成**: 最高頻率 ({}) 已滿足目標 (<= {}).", count, n);
                    break;
                }
            }

            if count <= 1 {
                println!("🛑 **最高頻率僅為 1，停止合併**");
                break;
            }

            step += 1;
            println!(
                "**Step {}**: 最常見組合 `{} + {}` 出現 **{}** 次",
                step, bigram.0, bigram.1, count
            );
            tokens = merge_tokens(&tokens, &bigram);
            println!("  ├─ 合併後 Token 數量: {}", tokens.len());
        } else {
            println!("🛑 **找不到可合併的 bigram，演算法結束**");
            break;
        }
    }

    println!("═══════════════════════════════════════");
    tokens
}

// CLI 參數定義

#[derive(Parser, Debug)]
#[command(author, version, about = "一個實現 BPE 演算法的標記器")]
struct Cli {
    /// 輸入來源：檔案或直接的文字
    #[command(flatten)]
    input: InputSource,

    ///執行的子命令：`percentage` 或 `frequency`
    #[command(subcommand)]
    command: Command,
}

#[derive(Args, Debug)]
struct InputSource {
    /// 從檔案讀取輸入
    #[arg(short, long, global = true, conflicts_with = "text")]
    file: Option<String>,

    /// 直接從命令列讀取文字
    #[arg(short, long, global = true)]
    text: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// 當 Token 數量達到原始數量的目標百分比時停止
    Percentage {
        /// 目標百分比 (1-100)。
        #[arg(short = 'p', long, default_value_t = 70, value_parser = clap::value_parser!(u8).range(1..=100))]
        value: u8,
    },
    /// 當最常見組合的頻率小於或等於 n 時停止
    Frequency {
        /// 頻率閾值 (必須 >= 2)。
        #[arg(short = 'n', long)]
        value: usize,
    },
}

fn main() {
    let cli = Cli::parse();

    // 手動檢查是否提供了輸入
    if cli.input.file.is_none() && cli.input.text.is_none() {
        eprintln!("❌ 錯誤：必須提供輸入。請使用 --file <檔案路徑> 或 --text <文字>。");
        eprintln!("\n使用 --help 查看更多資訊。");
        process::exit(1);
    }

    let text = match (cli.input.file, cli.input.text) {
        (Some(path), None) => fs::read_to_string(&path).unwrap_or_else(|err| {
            eprintln!("❌ 讀取檔案失敗 '{}': {}", path, err);
            process::exit(1);
        }),
        (None, Some(text)) => text,
        _ => unreachable!(), // Clap 的 conflicts_with 和上面的手動檢查會處理所有情況
    };

    if text.trim().is_empty() {
        println!("⚠️ 輸入文本為空，無需處理。");
        return;
    }

    let condition = match cli.command {
        Command::Percentage { value } => StoppingCondition::Percentage(value),
        Command::Frequency { value } => {
            if value < 2 {
                eprintln!("❌ 錯誤：frequency 的值必須大於或等於 2。");
                process::exit(1);
            }
            StoppingCondition::Frequency(value)
        }
    };

    let initial_chars = text.chars().count();
    let final_tokens = run_bpe(&text, condition);

    println!("🎯 **最終結果**: [{}]", format_tokens(&final_tokens));
    println!("Token 數量: {} → {}", initial_chars, final_tokens.len());
}
