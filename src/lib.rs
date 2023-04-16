mod components;
pub mod ecs;
mod engine;
mod event_handler;
mod input_handler;
mod math;
mod new_year;
mod render_systems;
mod renderer;
mod sdl_fronts;
mod texture_manager;
mod thread_pool;
mod window_config;

pub use components::*;
pub use ecs::*;
pub use engine::*;
pub use event_handler::*;
pub use input_handler::*;
pub use math::*;
pub use new_year::*;
pub use sdl_fronts::*;
pub use texture_manager::*;
pub use window_config::*;

// Re-export
pub use sdl2::gfx::primitives::DrawRenderer;
pub use sdl2::pixels::Color;
pub use sdl2::{render::Canvas, video::Window};
