use std::env;
use std::process;
use crate::approximate_fraction;

/// CLI版のエントリーポイント
pub fn run_cli() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("使用方法: bunka <小数点数> [オプション]");
        eprintln!("例) bunka 0.142857  ->  1/7");
        process::exit(1);
    }

    let val_str = &args[1];
    if val_str == "--help" || val_str == "-h" {
        print_help();
        process::exit(0);
    }
    if val_str == "--version" || val_str == "-v" || val_str == "-V" {
        println!("bunka {}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    let val: f64 = match val_str.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("エラー: '{}' は無効な浮動小数点数です", val_str);
            process::exit(1);
        }
    };

    let mut max_den = 100_000u64;
    let mut tolerance = 1e-6f64;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--max-den" | "-d" => {
                if i + 1 < args.len() {
                    max_den = match args[i + 1].parse() {
                        Ok(n) if n > 0 => n,
                        _ => {
                            eprintln!("エラー: 無効な最大分母指定です '{}'", args[i + 1]);
                            process::exit(1);
                        }
                    };
                    i += 2;
                } else {
                    eprintln!("エラー: オプション '{}' に値が指定されていません", args[i]);
                    process::exit(1);
                }
            }
            "--tolerance" | "-t" => {
                if i + 1 < args.len() {
                    tolerance = match args[i + 1].parse() {
                        Ok(n) if n > 0.0 => n,
                        _ => {
                            eprintln!("エラー: 無効な許容誤差指定です '{}'", args[i + 1]);
                            process::exit(1);
                        }
                    };
                    i += 2;
                } else {
                    eprintln!("エラー: オプション '{}' に値が指定されていません", args[i]);
                    process::exit(1);
                }
            }
            _ => {
                eprintln!("エラー: 未知のオプションです '{}'", args[i]);
                process::exit(1);
            }
        }
    }

    let (num, den, _) = approximate_fraction(val, max_den, tolerance);
    println!("{}/{}", num, den);
}

fn print_help() {
    println!("bunka - 小数点数から分数への近似変換ツール");
    println!();
    println!("使用方法:");
    println!("    bunka <小数点数> [オプション]");
    println!();
    println!("引数:");
    println!("    <小数点数>        分数に近似変換したい浮動小数点数");
    println!();
    println!("オプション:");
    println!("    -d, --max-den <値>  近似計算に使用する最大分母 (デフォルト: 100,000)");
    println!("    -t, --tolerance <値> 近似計算の許容誤差 (デフォルト: 1e-6)");
    println!("    -h, --help        このヘルプメッセージを表示して終了します");
    println!("    -v, -V, --version バージョン情報を表示して終了します");
    println!();
    println!("使用例:");
    println!("    bunka 0.142857   -> 1/7");
    println!("    bunka 3.14159265 -d 1000 -> 355/113");
}
