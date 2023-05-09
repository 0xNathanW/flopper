use crate::{Street, ConfigError};
use super::BetSizings;

// Tree config is created by the user and used to build the action tree.
#[derive(Clone, Default, Debug)]
pub struct TreeConfig {

    pub initial_street: Street,
    
    pub starting_pot: i32,
    
    pub effective_stack: i32,
    
    pub rake: f64,
    
    pub rake_cap: f64,
    
    pub bet_sizings: BetSizings,
    
    pub add_all_in_threshold: f64,

    pub force_all_in_threshold: f64,
}

impl TreeConfig {
    // Verifies config parameters.
    pub fn verify(&self) -> Result<(), ConfigError> {

        if self.rake < 0.0 || self.rake > 1.0 {
            return Err(ConfigError::InvalidInput(
                "Rake".to_string(), "Must be between 0 and 1.".to_string()
            ));
        }
        if self.rake_cap < 0.0 {
            return Err(ConfigError::InvalidInput(
                "Rake cap".to_string(), "Must be positive.".to_string()
            ));
        }
        if self.add_all_in_threshold < 0.0 {
            return Err(ConfigError::InvalidInput(
                "Add all in threshold".to_string(), "Must be positive.".to_string()
            ));
        }
        if self.force_all_in_threshold < 0.0 {
            return Err(ConfigError::InvalidInput(
                "Force all in threshold".to_string(), "Must be positive.".to_string()
            ));
        }

        Ok(())
    }
}