#[cfg(not(feature = "gui"))]
use std::env;
#[cfg(not(feature = "gui"))]
use std::process;

#[cfg(not(feature = "gui"))]
fn main() {
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

#[cfg(not(feature = "gui"))]
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

/// 連分数展開アルゴリズムによる分数近似
fn approximate_fraction(value: f64, max_denominator: u64, tolerance: f64) -> (i64, u64, f64) {
    if value == 0.0 {
        return (0, 1, 0.0);
    }

    let sign = if value < 0.0 { -1 } else { 1 };
    let target = value.abs();

    let mut h1 = 1i64;
    let mut h2 = 0i64;
    let mut k1 = 0u64;
    let mut k2 = 1u64;

    let mut r = target;
    let mut a = r.floor() as i64;
    let mut step = 0;

    loop {
        let h = a * h1 + h2;
        let k = (a as u64) * k1 + k2;

        if k > max_denominator {
            break;
        }

        h2 = h1;
        h1 = h;
        k2 = k1;
        k1 = k;

        let approx_value = (h1 as f64 / k1 as f64) * (sign as f64);
        let error = (value - approx_value).abs();

        if error <= tolerance || (r - a as f64).abs() < 1e-11 {
            break;
        }

        let diff = r - a as f64;
        if diff.abs() < 1e-11 {
            break;
        }
        r = 1.0 / diff;
        a = r.floor() as i64;
        
        step += 1;
        if step > 50 {
            break;
        }
    }

    let final_approx = (h1 as f64 / k1 as f64) * (sign as f64);
    (h1 * sign, k1, (value - final_approx).abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approximate_fraction_positive() {
        let (num, den, _) = approximate_fraction(0.142857, 100000, 1e-6);
        assert_eq!(num, 1);
        assert_eq!(den, 7);

        let (num, den, _) = approximate_fraction(0.333333, 100000, 1e-6);
        assert_eq!(num, 1);
        assert_eq!(den, 3);

        let (num, den, _) = approximate_fraction(3.14159265, 100000, 1e-6);
        assert_eq!(num, 355);
        assert_eq!(den, 113);
    }

    #[test]
    fn test_approximate_fraction_zero() {
        let (num, den, _) = approximate_fraction(0.0, 100000, 1e-6);
        assert_eq!(num, 0);
        assert_eq!(den, 1);
    }

    #[test]
    fn test_approximate_fraction_negative() {
        let (num, den, _) = approximate_fraction(-0.5, 100000, 1e-6);
        assert_eq!(num, -1);
        assert_eq!(den, 2);
    }
}

// ==========================================
// 2. GUI版ソースコード (egui_gui.rs)
// ==========================================
#[cfg(feature = "gui")]
use eframe::egui;

#[cfg(feature = "gui")]
struct BunkaGuiApp {
    input_str: String,
    numerator: i64,
    denominator: u64,
    max_den: u64,
    tolerance_exp: i32,
}

#[cfg(feature = "gui")]
impl Default for BunkaGuiApp {
    fn default() -> Self {
        Self {
            input_str: "0.142857".to_owned(),
            numerator: 1,
            denominator: 7,
            max_den: 100000,
            tolerance_exp: -6,
        }
    }
}

#[cfg(feature = "gui")]
impl BunkaGuiApp {
    fn recalculate(&mut self) {
        if let Ok(val) = self.input_str.parse::<f64>() {
            let tol = 10f64.powi(self.tolerance_exp);
            let (num, den, _) = approximate_fraction(val, self.max_den, tol);
            self.numerator = num;
            self.denominator = den;
        }
    }
}

#[cfg(feature = "gui")]
impl eframe::App for BunkaGuiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // CPU負荷低減のため1秒に1回更新
        ui.ctx().request_repaint_after(std::time::Duration::from_secs(1));

        // タイトルバー代わりのドラッグ移動グリップ領域
        let response = ui.horizontal(|ui| {
            ui.heading("BUNKA - DECIMAL TO FRACTION");
        }).response;

        if response.dragged() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Decimal:");
            if ui.text_edit_singleline(&mut self.input_str).changed() {
                self.recalculate();
            }
        });

        let old_max_den = self.max_den;
        let old_tol_exp = self.tolerance_exp;

        ui.add(egui::Slider::new(&mut self.max_den, 10..=100000).text("Max Den"));
        ui.add(egui::Slider::new(&mut self.tolerance_exp, -9..=-1).text("Tolerance 10^x"));

        if old_max_den != self.max_den || old_tol_exp != self.tolerance_exp {
            self.recalculate();
        }

        ui.separator();
        ui.vertical_centered(|ui| {
            ui.label("RESULT (IMPACT)");
            ui.heading(format!("{}/{}", self.numerator, self.denominator));
        });
    }
}

#[cfg(feature = "gui")]
fn main() {
    use windows::Win32::System::Threading::CreateMutexW;
    use windows::Win32::Foundation::GetLastError;
    use windows::Win32::Foundation::ERROR_ALREADY_EXISTS;

    // Mutexの名前を作成して二重起動チェック
    unsafe {
        let _handle = CreateMutexW(None, true, windows::core::w!("Global\\BunkaGuiAppMutex"));
        if GetLastError() == ERROR_ALREADY_EXISTS {
            return;
        }
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "BUNKA - DECIMAL TO FRACTION",
        options,
        Box::new(|_cc| Ok(Box::new(BunkaGuiApp::default()))),
    ).unwrap();
}