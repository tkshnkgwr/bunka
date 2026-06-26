use eframe::egui;
use crate::approximate_fraction;

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

impl eframe::App for BunkaGuiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // CPU負荷低減のため1秒に1回更新
        ui.ctx().request_repaint_after(std::time::Duration::from_secs(1));

        // 透過ウィンドウに対応するため、角丸の半透明カスタムパネルで覆う
        let custom_frame = egui::Frame {
            fill: egui::Color32::from_rgba_unmultiplied(20, 20, 28, 235), // 深みのある半透明ダークグレー
            corner_radius: egui::CornerRadius::same(12),
            shadow: egui::Shadow {
                offset: [0, 4],
                blur: 16,
                spread: 0,
                color: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 120),
            },
            inner_margin: egui::Margin::same(14),
            stroke: egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 15)), // 微かな境界線
            ..Default::default()
        };

        custom_frame.show(ui, |ui| {
            // ヘッダー（タイトルドラッグ領域 & 閉じるボタン）
            let header_response = ui.horizontal(|ui| {
                ui.heading(
                    egui::RichText::new("BUNKA")
                        .font(egui::FontId::proportional(15.0))
                        .strong()
                        .color(egui::Color32::from_rgb(140, 160, 255))
                );
                ui.label(
                    egui::RichText::new("APPROXIMATION")
                        .font(egui::FontId::proportional(11.0))
                        .color(egui::Color32::from_rgb(130, 130, 150))
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let close_btn = ui.add(
                        egui::Button::new(
                            egui::RichText::new("X")
                                .font(egui::FontId::proportional(13.0))
                                .color(egui::Color32::from_rgb(220, 100, 100))
                        )
                        .frame(false)
                    );
                    if close_btn.clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            }).response;

            // ヘッダー部分のドラッグでウィンドウを移動可能にする
            let drag_id = ui.make_persistent_id("header_drag");
            let drag_response = ui.interact(header_response.rect, drag_id, egui::Sense::drag());
            if drag_response.dragged() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
            }

            ui.add_space(4.0);
            ui.separator();
            ui.add_space(8.0);

            // 小数値入力
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Decimal:").color(egui::Color32::from_rgb(200, 200, 210)));
                let text_edit = egui::TextEdit::singleline(&mut self.input_str)
                    .margin(egui::vec2(6.0, 4.0))
                    .desired_width(180.0);
                if ui.add(text_edit).changed() {
                    self.recalculate();
                }
            });

            ui.add_space(8.0);

            let old_max_den = self.max_den;
            let old_tol_exp = self.tolerance_exp;

            // 最大分母スライダー
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Max Denom:").color(egui::Color32::from_rgb(170, 170, 180)));
                ui.add(egui::Slider::new(&mut self.max_den, 10..=100000).show_value(true));
            });

            // 許容誤差スライダー
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Tolerance:").color(egui::Color32::from_rgb(170, 170, 180)));
                ui.add(egui::Slider::new(&mut self.tolerance_exp, -9..=-1).show_value(true).text("10^x"));
            });

            if old_max_den != self.max_den || old_tol_exp != self.tolerance_exp {
                self.recalculate();
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(6.0);

            // 結果表示
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("RESULT").font(egui::FontId::proportional(11.0)).color(egui::Color32::from_rgb(130, 130, 150)));
                ui.add_space(2.0);

                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center).with_main_align(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(format!("{}/{}", self.numerator, self.denominator))
                            .font(egui::FontId::proportional(26.0))
                            .strong()
                            .color(egui::Color32::from_rgb(100, 240, 160))
                    );

                    ui.add_space(6.0);

                    let copy_btn = ui.add(
                        egui::Button::new(egui::RichText::new("Copy").font(egui::FontId::proportional(12.0)))
                            .frame(true)
                    );
                    if copy_btn.clicked() {
                        ui.ctx().copy_text(format!("{}/{}", self.numerator, self.denominator));
                    }
                });
            });
        });
    }
}

/// GUI版のエントリーポイント
pub fn run_gui() {
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
            .with_transparent(true)
            .with_always_on_top()
            .with_inner_size([320.0, 220.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "BUNKA - DECIMAL TO FRACTION",
        options,
        Box::new(|cc| {
            // 背景パネル自体を完全に透明に設定して、カスタムパネルの角丸透過が映えるようにする
            let mut visuals = egui::Visuals::dark();
            visuals.panel_fill = egui::Color32::TRANSPARENT;
            
            // ウィジェットの細部カスタマイズ
            visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(30, 30, 40);
            visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(45, 45, 60);
            visuals.widgets.active.bg_fill = egui::Color32::from_rgb(55, 55, 75);
            visuals.selection.bg_fill = egui::Color32::from_rgb(100, 120, 240);
            
            cc.egui_ctx.set_visuals(visuals);
            Ok(Box::new(BunkaGuiApp::default()))
        }),
    ).unwrap();
}
