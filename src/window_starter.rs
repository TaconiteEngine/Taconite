use thiserror::*;

use crate::WindowConfig;

#[derive(Default)]
pub struct WindowStarter();

// TODO: Remove allowing dead code
#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum WindowError {
    #[error("There was no GPU detected.")]
    NoGPU,
    #[error("Creating a new window with Winit failed.")]
    WindowFailure,
    #[error("Creating a new wgpu surface failed.")]
    SurfaceFailure,
}

impl WindowStarter {
    pub fn create_window(&mut self, window_config: WindowConfig) -> Result<(), WindowError> {
        todo!("Make a new window.")
    }
}
