mod app {
    use eframe::CreationContext;
    use egui::{Button, TextEdit, Ui, Visuals};

    pub struct App {
        view: fn(app: &mut App, ui: &mut Ui),
        pgn: String,
        half_move: String,
        answer: String,
    }

    impl Default for App {
        fn default() -> Self {
            Self {
                view: Self::main_view,
                pgn: String::new(),
                half_move: String::new(),
                answer: String::new(),
            }
        }
    }

    impl App {
        /// Called once before the first frame.
        pub fn new(cc: &CreationContext<'_>) -> Self {
            cc.egui_ctx.set_visuals(Visuals::light());
            Default::default()
        }

        fn main_view(&mut self, ui: &mut egui::Ui) {
            if ui.add(Button::new("Add card")).clicked() {
                self.view = Self::add_view;
            }
            ui.add(Button::new("Edit card"));
            ui.add(Button::new("Review"));
        }

        fn add_view(&mut self, ui: &mut egui::Ui) {
            let pgn_input = TextEdit::multiline(&mut self.pgn)
                .desired_rows(12)
                .hint_text("Enter PGN");
            pgn_input.show(ui);

            let half_move = TextEdit::singleline(&mut self.half_move)
                .hint_text("Half Move");
            half_move.show(ui);

            let answer =
                TextEdit::singleline(&mut self.answer).hint_text("Answer");
            answer.show(ui);

            ui.add(Button::new("Add"));
            if ui.add(Button::new("Done")).clicked() {
                self.view = Self::main_view;
            }
        }
    }

    impl eframe::App for App {
        /// Called each time the UI needs repainting, which may be many times
        /// per second. Put your widgets into a `SidePanel`, `TopPanel`,
        /// `CentralPanel`, `Window` or `Area`.
        fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
            #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                // The top panel is often a good place for a menu bar:
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            frame.close();
                        }
                    });
                });
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    (self.view)(self, ui);
                })
            });
        }
    }
}

const APP_NAME: &str = "chanki";

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    )
}
