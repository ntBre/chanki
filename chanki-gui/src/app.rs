use std::{error::Error, io::Read, path::Path, str::FromStr};

use chanki::{
    deck::{Card, Deck},
    pgn::Pgn,
    DECK_PATH,
};
use eframe::CreationContext;
use egui::{Button, TextEdit, TextureHandle, TextureOptions, Ui, Visuals};

pub struct App {
    view: fn(app: &mut App, ui: &mut Ui),
    pgn: String,
    half_move: String,
    answer: String,
    deck: Deck,
    cur_card: Option<TextureHandle>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            view: Self::main_view,
            pgn: String::new(),
            half_move: String::new(),
            answer: String::new(),
            deck: Deck::load(DECK_PATH).unwrap_or_else(|_| Deck::default()),
            cur_card: None,
        }
    }
}

/// load a PNG image from `path` and convert it to an `egui::ColorImage`.
/// adapted from egui_extras
pub fn load_image(
    path: impl AsRef<Path>,
) -> Result<egui::ColorImage, Box<dyn Error>> {
    let mut f = std::fs::File::open(path)?;
    let mut image_bytes = Vec::new();
    f.read_to_end(&mut image_bytes)?;
    let image =
        image::load_from_memory(&image_bytes).map_err(|err| err.to_string())?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
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
        if ui.add(Button::new("Edit card")).clicked() {
            self.view = Self::edit_view;
        }
        if ui.add(Button::new("Review")).clicked() {
            self.view = Self::review_prompt;
        }
    }

    fn add_view(&mut self, ui: &mut egui::Ui) {
        self.add_card(ui);

        if ui.add(Button::new("Add")).clicked() {
            self.deck.push(Card::new(
                &Pgn::from_str(&self.pgn).unwrap(),
                self.half_move.parse().unwrap(),
                "card.png".to_owned(),
                std::mem::take(&mut self.answer),
            ));
            self.deck.dump(DECK_PATH).unwrap();
            self.pgn.clear();
            self.half_move.clear();
        }

        if ui.add(Button::new("Done")).clicked() {
            self.view = Self::main_view;
        }
    }

    fn add_card(&mut self, ui: &mut Ui) {
        let pgn_input = TextEdit::multiline(&mut self.pgn)
            .desired_rows(12)
            .hint_text("Enter PGN");
        pgn_input.show(ui);

        let half_move =
            TextEdit::singleline(&mut self.half_move).hint_text("Half Move");
        half_move.show(ui);

        let answer = TextEdit::singleline(&mut self.answer).hint_text("Answer");
        answer.show(ui);
    }

    fn edit_view(&mut self, ui: &mut egui::Ui) {
        self.show_card(ui);
        self.add_card(ui);
        if ui.add(Button::new("Update")).clicked() {
            self.deck.cards[0] = Card::new(
                &Pgn::from_str(&self.pgn).unwrap(),
                self.half_move.parse().unwrap(),
                "card.png".to_owned(),
                std::mem::take(&mut self.answer),
            );
            self.deck.dump(DECK_PATH).unwrap();
            self.pgn.clear();
            self.half_move.clear();
        }
        if ui.add(Button::new("Done")).clicked() {
            self.view = Self::main_view;
        }
    }

    fn review_prompt(&mut self, ui: &mut egui::Ui) {
        self.show_card(ui);
        if ui.add(Button::new("Check")).clicked() {
            self.view = Self::review_answer;
        }
    }

    fn show_card(&mut self, ui: &mut Ui) {
        if self.cur_card.is_none() {
            self.cur_card = Some(ui.ctx().load_texture(
                "the card",
                load_image(&self.deck.cards[0].pgn).unwrap(),
                TextureOptions::default(),
            ));
        }
        ui.image(self.cur_card.as_ref().unwrap(), [320., 320.]);
    }

    fn review_answer(&mut self, ui: &mut egui::Ui) {
        ui.label(format!("Answer: {}", self.deck.cards[0].answer));
        ui.add(Button::new("1"));
        ui.add(Button::new("2"));
        ui.add(Button::new("3"));
        ui.add(Button::new("4"));
        ui.add(Button::new("5"));
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
