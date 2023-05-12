use crate::config::*;

enum State {
    Config,
}

pub struct Flopper {
    state:  State,
    config: Config,
}

impl Flopper {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Flopper {
        Flopper {
            state:  State::Config,
            config: Config::default(),
        }
    }
}

impl eframe::App for Flopper {
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

            match self.state {

                State::Config => {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        
                        let h = ui.available_height();
                        let w = ui.available_width();
                        // OOP.
                        ui.group(|ui| {
                            ui.set_height(h / 3.0);
                            ui.set_width(w);
                            ui.spacing_mut().item_spacing = egui::Vec2::new(15.0, 15.0);
                            self.config.players[0].show(ui, true);
                        });
                        // IP.
                        ui.group(|ui| {
                            ui.set_height(h / 3.0);
                            ui.set_width(w);
                            ui.spacing_mut().item_spacing = egui::Vec2::new(15.0, 15.0);
                            self.config.players[1].show(ui, false);
                        });
                        // Board/Other.
                        ui.group(|ui| {
                            ui.set_height(h / 3.0);
                            ui.set_width(w);
                            ui.spacing_mut().item_spacing = egui::Vec2::new(15.0, 15.0);
                            
                        });
                    });
                },

                _ => unimplemented!(),
            }

        

    }
}