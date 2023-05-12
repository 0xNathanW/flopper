use std::collections::HashMap;
use egui::{widgets::Button, Slider};
use poker::{card::{RANKS, Rank}, range::RangeParseError};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RangeElem {
    Pair(Rank),
    Suited(Rank, Rank),
    Offsuit(Rank, Rank),
}

#[derive(Debug, Default)]
pub struct RangeInput {
    pub range: HashMap<RangeElem, f64>,
    pub name: String,
    pub weight: f64,
}

impl RangeInput {

    pub fn range_selector(&mut self, ui: &mut egui::Ui, hero: bool) {

        ui.spacing_mut().item_spacing = egui::vec2(5.0, 5.0);
        let name = if hero { "Hero" } else { "Villain" }.to_string();
        ui.heading(format!("{} Range", name));
        
        ui.group(|ui| { egui::Grid::new(format!("{:?}_Range", name))
            .spacing(egui::vec2(2.0, 2.0))
            .striped(true)
            .num_columns(13)
            .max_col_width(40.0)
            .min_col_width(40.0)
            .min_row_height(40.0)
            .show(ui, |ui| {
                
            for i in RANKS.iter().rev() {
                for j in RANKS.iter().rev() {
                    if i == j {
                        let b = Button::new(format!("{}{}", i, j))
                            .min_size(egui::vec2(40.0, 40.0))
                            .sense(egui::Sense::click_and_drag())
                            .fill(
                                match self.range.get(&RangeElem::Pair(*i)) {
                                    Some(w) => egui::Color32::from_rgba_unmultiplied(0, 150, 0, (255 as f64 * w) as u8),
                                    None    => egui::Color32::TRANSPARENT,
                                }
                            );
                        
                        ui.vertical_centered(|ui| {
                            ui.add(b);
                        });
                        // if ui.add(b).clicked() {
                        //     if let Some(_) = self.range.get(&RangeElem::Pair(*i)) {
                        //         self.range.remove(&RangeElem::Pair(*i));
                        //     } else {
                        //         self.range.insert(RangeElem::Pair(*i), self.weight);
                        //     }
                        // }
                    
                    } else if i > j {
                        let b = Button::new(format!("{}{}s", i, j))
                            .min_size(egui::vec2(40.0, 40.0))
                            .sense(egui::Sense::click_and_drag())
                            .fill(
                                match self.range.get(&RangeElem::Suited(*i, *j)) {
                                    Some(w) => egui::Color32::from_rgba_unmultiplied(0, 0, 150, (255 as f64 * w) as u8),
                                    None    => egui::Color32::TRANSPARENT,
                                }
                            );
                        if ui.add(b).clicked() {
                            if let Some(_) = self.range.get(&RangeElem::Suited(*i, *j)) {
                                self.range.remove(&RangeElem::Suited(*i, *j));
                            } else {
                                self.range.insert(RangeElem::Suited(*i, *j), self.weight);
                            }
                        }
                    } else {
                        let b = Button::new(format!("{}{}o", i, j))
                            .min_size(egui::vec2(40.0, 40.0))
                            .sense(egui::Sense::click_and_drag())
                            .fill(
                                match self.range.get(&RangeElem::Offsuit(*i, *j)) {
                                    Some(w) => egui::Color32::from_rgba_unmultiplied(150, 0, 0, (255 as f64 * w) as u8),
                                    None    => egui::Color32::TRANSPARENT,
                                }
                            );
                        
                        if ui.add(b).clicked() {
                            if let Some(_) = self.range.get(&RangeElem::Offsuit(*i, *j)) {
                                self.range.remove(&RangeElem::Offsuit(*i, *j));
                            } else {
                                self.range.insert(RangeElem::Offsuit(*i, *j), self.weight);
                            }
                        }
                    }
                }
                ui.end_row();
            }
        })});
        ui.add_space(10.0);
        ui.add(
            Slider::new(&mut self.weight, 0.0..=1.0)
                .clamp_to_range(true)
                .text("Weight")
        );
    }
}

pub fn from_str(s: &str) -> Result<HashMap<RangeElem, f64>, RangeParseError> {
    if s.len() == 0 {
        return Ok(HashMap::new());
    }

    let mut range = HashMap::new();
    if s.starts_with("[") {
        let mut chars = s.chars();
        loop {
            if let Some(c) = chars.next() {
                
                if c == ' ' {
                    continue;
                } else if c != '[' {
                    return Err(RangeParseError::UnexpectedToken(c, "[".to_string()));
                }

                let mut weight_str_pre = String::new();
                loop {
                    let c = chars.next().ok_or(RangeParseError::UnexpectedEOF)?;
                    if c == ']' {
                        break;
                    }
                    weight_str_pre.push(c);
                }

                let weight = str::parse::<f32>(&weight_str_pre.as_str())?;
                if weight < 0.0 || weight > 1.0 {
                    return Err(RangeParseError::InvalidWeight(weight));
                }

                let mut r = String::new();
                loop {
                    let c = chars.next().ok_or(RangeParseError::UnexpectedEOF)?;
                    // Check for weight closer.
                    if c == '[' {
                        break;
                    }
                    r.push(c);
                }

                if chars.next().ok_or(RangeParseError::UnexpectedEOF)? != '/' {
                    return Err(RangeParseError::UnexpectedToken(c, "/".to_string()));
                }

                let mut weight_str_suf = String::new();
                // Parse weight closer
                loop {
                    let c = chars.next().ok_or(RangeParseError::UnexpectedEOF)?;
                    if c == ']' {
                        break;
                    }
                    weight_str_suf.push(c);
                }

                if weight_str_pre != weight_str_suf {
                    return Err(RangeParseError::Custom("Weight open and close are not equal".to_string()));
                }

                // parse weight
                parse_weight(&mut range, &r, weight as f64)?;
            } else {
                break;
            }
        }
    } else {
        // Unweighted range.
        parse_weight(&mut range, s, 1.0)?;
    }

    Ok(range)
}

// Parse all elements for a given weight.
fn parse_weight(range: &mut HashMap<RangeElem, f64>, input: &str, weight: f64) -> Result<(), RangeParseError> {

    for elem in input.split(",").map(|s| s.trim()) {
        let mut chars = elem.chars();

        match elem.len() {

            // Single Pair
            2 => {
                let rank_1 = Rank::from_str(chars.next().unwrap())?;
                let rank_2 = Rank::from_str(chars.next().unwrap())?;
                if rank_1 != rank_2 {
                    return Err(RangeParseError::NoSuitedness(rank_1, rank_2));
                }
                range.insert(RangeElem::Pair(rank_1), weight);
            },

            // Pair+, Suited, Offsuit
            3 => {
                let mut rank_1 = Rank::from_str(chars.next().unwrap())?;
                let mut rank_2 = Rank::from_str(chars.next().unwrap())?;
                if rank_2 > rank_1 {
                    std::mem::swap(&mut rank_1, &mut rank_2);
                }

                // Pair+
                if rank_1 == rank_2 {
                    let p = chars.next().unwrap();
                    if p == '+' {
                        for rank in rank_1 as u8..=Rank::Ace as u8 {
                            range.insert(RangeElem::Pair(rank.into()), weight);
                        }
                    } else {
                        return Err(RangeParseError::UnexpectedToken(p, "+".to_string()));
                    }
                } 

                // Suited, Offsuit
                else {
                    let suitedness = chars.next().unwrap();
                    match suitedness {
                        's' => range.insert(RangeElem::Suited(rank_1, rank_2), weight),
                        'o' => range.insert(RangeElem::Offsuit(rank_1, rank_2), weight),
                        _ => return Err(RangeParseError::UnexpectedToken(suitedness, "s for suited or o for offsuit".to_string())),
                    };
                }
            },

            // Suited+, Offsuit+
            4 => {
                let mut rank_1 = Rank::from_str(chars.next().unwrap())?;
                let mut rank_2 = Rank::from_str(chars.next().unwrap())?;
                if rank_2 > rank_1 {
                    std::mem::swap(&mut rank_1, &mut rank_2);
                }
                let suitedness = chars.next().unwrap();

                if chars.next().unwrap() != '+' {
                    return Err(RangeParseError::UnexpectedToken('+', "+".to_string()));
                }

                match suitedness {
                    's' => {
                        for rank in rank_2 as u8..rank_1 as u8 {
                            range.insert(RangeElem::Suited(rank_1, rank.into()), weight);
                        }
                    },
                    'o' => {
                        for rank in rank_2 as u8..rank_1 as u8 {
                            range.insert(RangeElem::Offsuit(rank_1, rank.into()), weight);
                        }
                    },
                    _ => return Err(RangeParseError::UnexpectedToken(suitedness, "s for suited or o for offsuit".to_string())),
                }
            },

            // Pair - Pair
            5 => {
                let rank_1 = Rank::from_str(chars.next().unwrap())?;
                let rank_2 = Rank::from_str(chars.next().unwrap())?;

                if chars.next().unwrap() != '-' {
                    return Err(RangeParseError::UnexpectedToken('-', "-".to_string()));
                }

                let rank_3 = Rank::from_str(chars.next().unwrap())?;
                let rank_4 = Rank::from_str(chars.next().unwrap())?;

                if rank_1 != rank_2 || rank_3 != rank_4 {
                    return Err(RangeParseError::NoSuitedness(rank_1, rank_2));
                }

                let max_rank = rank_1.max(rank_3);
                let min_rank = rank_1.min(rank_3);

                for rank in min_rank as u8..=max_rank as u8 {
                    range.insert(RangeElem::Pair(rank.into()), weight);
                }
            },

            // No input for 6 characters.

            // Suited - Suited, Offsuit - Offsuit
            7 => {
                let rank_1 = Rank::from_str(chars.next().unwrap())?;
                let rank_2 = Rank::from_str(chars.next().unwrap())?;
                let max_rank_1 = rank_1.max(rank_2);
                let min_rank_1 = rank_1.min(rank_2);
                let suitedness_1 = chars.next().unwrap();

                if chars.next().unwrap() != '-' {
                    return Err(RangeParseError::UnexpectedToken('-', "-".to_string()));
                }

                let rank_3 = Rank::from_str(chars.next().unwrap())?;
                let rank_4 = Rank::from_str(chars.next().unwrap())?;
                let max_rank_2 = rank_3.max(rank_4);
                let min_rank_2 = rank_3.min(rank_4);
                let suitedness_2 = chars.next().unwrap();

                if suitedness_1 != suitedness_2 {
                    return Err(RangeParseError::UnexpectedToken(suitedness_2, "s for suited or o for offsuit".to_string()));
                }

                if max_rank_1 != max_rank_2 {
                    return Err(RangeParseError::UnexpectedToken(
                        format!("{}", max_rank_2).chars().next().unwrap(), 
                        format!("{}", max_rank_1).to_string())
                    );
                }

                if min_rank_1 > min_rank_2 {
                    return Err(RangeParseError::Custom("Min rank 1 is greater than min rank 2".to_string()));
                }

                match suitedness_1 {
                    's' => {
                        for rank in min_rank_1 as u8..=min_rank_2 as u8 {
                            range.insert(RangeElem::Suited(rank.into(), min_rank_2), weight);
                        }
                    },
                    'o' => {
                        for rank in min_rank_1 as u8..=min_rank_2 as u8 {
                            range.insert(RangeElem::Offsuit(rank.into(), min_rank_2), weight);
                        }
                    },
                    _ => return Err(RangeParseError::UnexpectedToken(suitedness_1, "s for suited or o for offsuit".to_string())),
                }
            },

            _ => panic!("Unexpected number of characters in range input: {}", input),
        }
    }

    Ok(())
}