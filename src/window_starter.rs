use thiserror::*;

use crate::WindowConfig;

#[derive(Default)]
pub struct WindowStarter();

#[derive(Debug, Error)]
pub enum WindowError {
    #[error("There was no GPU detected.")]
    NoGPU,
}

impl WindowStarter {
    pub fn begin(&mut self, window_config: WindowConfig) -> Result<(), WindowError> {
        todo!("Make a new window.")
    }
}
