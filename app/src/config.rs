use egui::{Ui, TextEdit, RichText, Vec2};
use gto::action::TreeConfig;
use poker::Board;
use crate::range::{RangeInput, self};

const POSITIONS: [&'static str; 6] = ["UTG", "HJ", "CO", "BTN", "SB", "BB"];

#[derive(Debug, Default)]
pub struct Config {
    pub tree_config: TreeConfig,
    pub players:     [Player; 2],
    pub board:       Board,
}

#[derive(Debug, Default)]
pub struct Player {
    pub oop: bool,
    pub range: RangeInput,
    pub position: usize,
}

impl Player {

    pub fn show(&mut self, ui: &mut Ui, ip: bool) {

        let title = if ip { "In Position" } else { "Out Of Position" };
        ui.heading(RichText::new(title).size(30.0));
        
        // Range.
        ui.horizontal(|ui| {
            ui.label(RichText::new("Range:").size(20.0));

            let resp = ui.add(TextEdit::singleline(&mut self.range.name).min_size(Vec2::new(50.0, 20.0)));
            if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                match range::from_str(&self.range.name) {
                    Ok(range) => self.range.range = range,
                    Err(err)  => self.range.name = format!("{}", err),
                }
            }
        });

        let h = ui.available_height();
        let w = ui.available_width();

        // Bet Sizings.
        ui.horizontal(|ui| {
            // Flop.
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.set_height(h);
                    ui.set_width(w / 3.0);
                    ui.heading(RichText::new("Flop Bets").size(25.0));
                    
                });
            });
            // Turn.
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.set_height(h);
                    ui.set_width(w / 3.0);
                    ui.heading(RichText::new("Turn Bets").size(25.0));
                });
            });
            
            // River.
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.set_height(h);
                    ui.set_width(w / 3.0);
                    ui.heading(RichText::new("River Bets").size(25.0));
                });
            });
        });

    }

}


