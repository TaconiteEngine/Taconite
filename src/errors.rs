use thiserror::*;

#[derive(Debug, Error)]
pub enum WindowError {
    #[error("There was no GPU detected.")]
    NoGPU,
    #[error("Creating a new window with Winit failed.")]
    WindowFailure,
    #[error("Creating a new wgpu surface failed.")]
    SurfaceFailure,
    #[error("Can't resize to this size.")]
    ResizeError,
}
