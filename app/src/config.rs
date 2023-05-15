use egui::{Ui, TextEdit, RichText, Vec2, ScrollArea, CollapsingHeader, DragValue, Button};
use gto::{action::{TreeConfig, BetSizingsStreet, BetSize}};
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
    // Whether the player is out of position.
    pub oop:        bool,
    // Ranges.
    pub range:      RangeInput,
    // Position on table.
    pub position:   usize,
    // Bet sizings for each street.
    pub bets:       [BetSizingsStreet; 3],
    // Whether to show the edit window for each bet/raise.
    pub edit:       [[bool; 2]; 3],
    // The current bet sizings being edited. 
    pub curr_sizes:  CurrentSizings,
}

// The current bet sizings being edited.
#[derive(Default, Debug)]
pub struct CurrentSizings {
    absolute:    i32,
    pot_scaled:  f64,
    prev_scaled: f64,
    geometric:   i32,
}

impl Player {

    pub fn new_players() -> [Player; 2] {
        [
            Player {
                oop:        true,
                range:      RangeInput::default(),
                position:   0,
                bets:       Default::default(),
                edit:       [[false; 2]; 3],
                curr_sizes:  Default::default(),
            },
            Player {
                oop:        false,
                range:      RangeInput::default(),
                position:   0,
                bets:       Default::default(),
                edit:       [[false; 2]; 3],
                curr_sizes:  Default::default(),
            },
        ]
    }

    pub fn show(&mut self, ui: &mut Ui) {

        ui.heading(RichText::new(if self.oop { "Out Of Position" } else { "In Position" }).size(30.0));
        
        // Range.
        ui.horizontal(|ui| {
            ui.label(RichText::new("Range:").size(20.0));
            // Input for ranges.
            let resp = ui.add(TextEdit::singleline(&mut self.range.name).min_size(Vec2::new(50.0, 20.0)));
            if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                match range::from_str(&self.range.name) {
                    Ok(range) => self.range.range = range,
                    Err(err)  => self.range.name = format!("{}", err),
                }
            }
        });

        let (w, h) = (ui.available_width() / 3.0, ui.available_height());

        // Bet Sizings.
        ui.horizontal(|ui| {
            // Flop.
            self.show_bets(ui, 0, (w, h));
            // Turn.
            self.show_bets(ui, 1, (w, h));
            // River.
            self.show_bets(ui, 2, (w, h));
        });

    }

    fn show_bets(&mut self, ui: &mut Ui, street: usize, size: (f32, f32)) {
        
        let opp = if self.oop { "IP" } else { "OOP" };
        let street_txt = match street {
            0 => "Flop",
            1 => "Turn",
            2 => "River",
            _ => unreachable!(),
        };

        ui.group(|ui| {

            ui.set_width(size.0);
            ui.set_height(size.1);

            ui.vertical(|ui| {
                ui.heading(RichText::new(format!("{} Bets", street_txt)).size(25.0));
                ui.columns(2, |cols| {

                    CollapsingHeader::new(RichText::new("Bets").size(15.0))
                        .id_source(format!("{}_{}_bets", opp, street))
                        .default_open(true)
                        .show(&mut cols[0], |ui| {
                            ScrollArea::vertical().show(ui, |ui| {
                                for bet in self.bets[street].bet.iter() {
                                    ui.label(RichText::new(format!("{:?}", bet)).size(15.0));
                                }
                            });
                    });

                    if cols[0].button(RichText::new("Add").size(15.0)).clicked() {
                        self.edit[street][0] = true;
                    }

                    if cols[0].button(RichText::new("Clear").size(15.0)).clicked() {
                        self.bets[street].bet.clear();
                    }

                    CollapsingHeader::new(RichText::new("Raises").size(15.0))
                        .id_source(format!("{}_{}_raises", opp, street))
                        .default_open(true)
                        .show(&mut cols[1], |ui| {
                            ScrollArea::vertical().show(ui, |ui| {
                                for raise in self.bets[street].raise.iter() {
                                    ui.label(RichText::new(format!("{:?}", raise)).size(15.0));
                                }
                            });
                    });

                    if cols[1].button(RichText::new("Add").size(15.0)).clicked() {
                        self.edit[street][1] = true;
                    };

                    if cols[1].button(RichText::new("Clear").size(15.0)).clicked() {
                        self.bets[street].raise.clear();
                    }
                });
            });
        });
    }

    pub fn edit_bets(&mut self, ctx: &egui::Context, street: usize, raise: usize) {
        egui::Window::new(format!("Add {} {} {}", 
            if self.oop { "OOP" } else { "IP" }, 
            match street {
                0 => "Flop",
                1 => "Turn",
                2 => "River",
                _ => unreachable!(),
            },
            if raise == 0 { "Bets" } else { "Raises" }
        ))
            .open(&mut self.edit[street][raise])
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.spacing_mut().item_spacing = egui::Vec2::new(15.0, 15.0);
                    
                    ui.columns(2, |cols| {

                        cols[0].horizontal(|ui| {
                            ui.label(RichText::new("Absolute:").size(20.0));
                            ui.add(DragValue::new(&mut self.curr_sizes.absolute)
                                .speed(1)
                                .clamp_range(1..=i32::MAX)
                                .suffix(" units")
                            );
    
                        });
                        if cols[1].button(RichText::new("Add").size(15.0)).clicked() {
                            if raise == 1 {
                                self.bets[street].raise.push(BetSize::Absolute(self.curr_sizes.absolute));
                            } else {
                                self.bets[street].bet.push(BetSize::Absolute(self.curr_sizes.absolute));
                            }
                        }
                        
                        cols[0].horizontal(|ui| {
                            ui.label(RichText::new("Pot Scaled:").size(20.0));
                            ui.add(DragValue::new(&mut self.curr_sizes.pot_scaled)
                                .speed(0.5)
                                .clamp_range(0.0..=f64::MAX)
                                .suffix("%")
                            );
    
                        });
                        if cols[1].button(RichText::new("Add").size(15.0)).clicked() {
                            if raise == 1 {
                                self.bets[street].raise.push(BetSize::PotScaled(self.curr_sizes.pot_scaled / 100.0));
                            } else {
                                self.bets[street].bet.push(BetSize::PotScaled(self.curr_sizes.pot_scaled / 100.0));
                            }
                        }
                        
                        cols[0].horizontal(|ui| {
                            ui.label(RichText::new("Previous Bet Scaled:").size(20.0));
                            ui.add(DragValue::new(&mut self.curr_sizes.prev_scaled)
                                .speed(0.01)
                                .clamp_range(1.0..=f64::MAX)
                                .suffix("x")
                            );
    
                        });
                        if cols[1].button(RichText::new("Add").size(15.0)).clicked() {
                            if raise == 1 {
                                self.bets[street].raise.push(BetSize::PrevScaled(self.curr_sizes.prev_scaled));
                            } else {
                                self.bets[street].bet.push(BetSize::PrevScaled(self.curr_sizes.prev_scaled));
                            }
                        }
                        
                        cols[0].horizontal(|ui| {
                            ui.label(RichText::new("Geometric:").size(20.0));
                            ui.add(DragValue::new(&mut self.curr_sizes.geometric)
                                .speed(1)
                                .clamp_range(0..=i32::MAX)
                            );
                            
                        });
                        if cols[1].button(RichText::new("Add").size(15.0)).clicked() {
                            if raise == 1 {
                                self.bets[street].raise.push(BetSize::Geometric(self.curr_sizes.geometric, 0.1));
                            } else {
                                self.bets[street].bet.push(BetSize::Geometric(self.curr_sizes.geometric, 0.1));
                            }
                        }
                    });
                });
            });

    }
}


