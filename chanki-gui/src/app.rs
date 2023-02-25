use std::{error::Error, io::Read, path::Path, str::FromStr};

use chanki::{
    deck::{Card, Deck, Quality},
    pgn::Pgn,
    DECK_PATH,
};
use eframe::CreationContext;
use egui::{Button, TextEdit, TextureHandle, TextureOptions, Ui, Visuals};
use rand::seq::SliceRandom;

struct CardImage {
    texture: TextureHandle,
    index: usize,
}

pub struct App {
    view: fn(app: &mut App, ui: &mut Ui),
    pgn: String,
    half_move: String,
    answer: String,
    deck: Deck,
    cur_card: Option<CardImage>,
    some_review: bool,
}

impl Default for App {
    fn default() -> Self {
        let deck = Deck::load(DECK_PATH).unwrap_or_else(|e| {
            eprintln!("error loading deck: {e}");
            Deck::default()
        });
        let some_review = deck.cards.iter().any(|c| c.is_due());
        Self {
            view: Self::main_view,
            pgn: String::new(),
            half_move: String::new(),
            answer: String::new(),
            deck,
            cur_card: None,
            some_review,
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

        if ui
            .add_enabled(self.some_review, Button::new("Review"))
            .clicked()
        {
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
            self.some_review = true;
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
        let idx = self.cur_card_index();
        self.show_card(ui, idx);
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
            self.some_review = true;
        }
        if ui.add(Button::new("Done")).clicked() {
            self.view = Self::main_view;
        }
        if ui
            .add_enabled(idx < self.deck.cards.len() - 1, Button::new("Next"))
            .clicked()
        {
            self.show_card(ui, idx + 1);
        }
        if ui.add_enabled(idx > 0, Button::new("Prev")).clicked() {
            self.show_card(ui, idx - 1);
        }
    }

    fn review_prompt(&mut self, ui: &mut egui::Ui) {
        let mut cards: Vec<usize> = self
            .deck
            .cards
            .iter()
            .enumerate()
            .filter_map(|(i, c)| if c.is_due() { Some(i) } else { None })
            .collect();
        cards.shuffle(&mut rand::thread_rng());
        if let Some(i) = cards.first() {
            self.show_card(ui, *i);
            if ui.add(Button::new("Check")).clicked() {
                self.view = Self::review_answer;
                return;
            }
        } else {
            self.some_review = false;
            self.view = Self::main_view;
        }
    }

    fn show_card(&mut self, ui: &mut Ui, index: usize) {
        let idx = self.cur_card_index();
        if self.cur_card.is_none() || index != idx {
            self.cur_card = Some(CardImage {
                texture: ui.ctx().load_texture(
                    "the card",
                    load_image(&self.deck.cards[index].pgn).unwrap(),
                    TextureOptions::default(),
                ),
                index,
            });
        }
        ui.image(&self.cur_card.as_ref().unwrap().texture, [320., 320.]);
    }

    fn review_answer(&mut self, ui: &mut egui::Ui) {
        let idx = self.cur_card_index();
        self.show_card(ui, idx);
        ui.label(format!("Answer: {}", self.deck.cards[0].answer));
        let idx = self.cur_card_index();
        if ui.add(Button::new("0")).clicked() {
            self.deck.cards[idx].update_card(Quality::Zero);
            self.deck.dump(DECK_PATH).unwrap();
            self.view = Self::review_prompt;
        }
        if ui.add(Button::new("1")).clicked() {
            self.deck.cards[idx].update_card(Quality::One);
            self.deck.dump(DECK_PATH).unwrap();
            self.view = Self::review_prompt;
        }
        if ui.add(Button::new("2")).clicked() {
            self.deck.cards[idx].update_card(Quality::Two);
            self.deck.dump(DECK_PATH).unwrap();
            self.view = Self::review_prompt;
        }
        if ui.add(Button::new("3")).clicked() {
            self.deck.cards[idx].update_card(Quality::Three);
            self.deck.dump(DECK_PATH).unwrap();
            self.view = Self::review_prompt;
        }
        if ui.add(Button::new("4")).clicked() {
            self.deck.cards[idx].update_card(Quality::Four);
            self.deck.dump(DECK_PATH).unwrap();
            self.view = Self::review_prompt;
        }
        if ui.add(Button::new("5")).clicked() {
            self.deck.cards[idx].update_card(Quality::Five);
            self.deck.dump(DECK_PATH).unwrap();
            self.view = Self::review_prompt;
        }
    }

    fn cur_card_index(&self) -> usize {
        if let Some(c) = &self.cur_card {
            c.index
        } else {
            0
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
