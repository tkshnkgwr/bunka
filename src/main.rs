// 3. 最新のソースコード (Rust)

// --- Cargo.toml ---
// [package]
// name = "bunka"
// version = "1.0.0"
// edition = "2021"
//
// [dependencies]
// eframe = "0.22.0" # GUI版を動かす場合

// ==========================================
// 1. CLI版ソースコード (main_cli.rs)
// ==========================================
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("使用方法: bunka <小数点数>");
        eprintln!("例) bunka 0.142857  ->  1/7");
        process::exit(1);
    }

    let val_str = &args[1];
    let val: f64 = match val_str.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("エラー: '{}' は無効な浮動小数点数です", val_str);
            process::exit(1);
        }
    };

    // 最大分母 100,000 / 誤差許容 1e-6 で計算
    let (num, den, _) = approximate_fraction(val, 100000, 1e-6);
    println!("{}/{}", num, den);
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

// ==========================================
// 2. GUI版ソースコード (egui_gui.rs)
// ==========================================
// [Rust egui による極小最前面デスクトップアプリ実装]
/*
use eframe::egui;

struct BunkaGuiApp {
    input_str: String,
    numerator: i64,
    denominator: u64,
    max_den: u64,
    tolerance_exp: i32,
}

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

impl eframe::App for BunkaGuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // CPU負荷低減のため1秒に1回更新
        ctx.request_repaint_after(std::time::Duration::from_secs(1));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("BUNKA - DECIMAL TO FRACTION");
            
            ui.horizontal(|ui| {
                ui.label("Decimal:");
                if ui.text_edit_singleline(&mut self.input_str).changed() {
                    self.recalculate();
                }
            });

            ui.add(egui::Slider::new(&mut self.max_den, 10..=100000).text("Max Den"));
            ui.add(egui::Slider::new(&mut self.tolerance_exp, -9..=-1).text("Tolerance 10^x"));

            ui.separator();
            ui.vertical_centered(|ui| {
                ui.label("RESULT (IMPACT)");
                ui.heading(format!("{}/{}", self.numerator, self.denominator));
            });
        });
    }
}
*/