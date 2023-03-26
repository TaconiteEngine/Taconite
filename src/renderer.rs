use crate::EventHandler;
use crate::TextureManager;
use crate::WindowConfig;
use crate::World;
use glow::HasContext;
use glow::NativeShader;
use log::info;
// use sdl2::{image::InitFlag, pixels::Color};
use std::sync::{Arc, Mutex};

pub struct Renderer {
    pub(crate) event_handler: EventHandler,
    texture_manager: Option<TextureManager>,
}

impl Renderer {
    pub fn new(world: Arc<Mutex<World>>) -> Renderer {
        info!("Creating a renderer");

        Renderer {
            event_handler: EventHandler::new(world),
            texture_manager: None,
        }
    }

    pub fn start_window(&mut self, window_config: WindowConfig) -> Result<(), String> {
        // Can't call within safe rust unfortuantely.
        unsafe {
            let (gl, shader_version, window, event_loop) = {
                let event_loop = glutin::event_loop::EventLoop::new();

                let window_builder = glutin::window::WindowBuilder::new()
                    .with_title(window_config.name)
                    .with_inner_size(glutin::dpi::LogicalSize::new(
                        window_config.width,
                        window_config.height,
                    ));

                // Create a window.
                let window = glutin::ContextBuilder::new()
                    .with_vsync(window_config.vsync)
                    .build_windowed(window_builder, &event_loop)
                    .unwrap()
                    .make_current()
                    .unwrap();

                // Create a GL context.
                let gl =
                    glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);

                (gl, "#version 410", window, event_loop)
            };

            let vertex_array = gl
                .create_vertex_array()
                .expect("Failed to create a new vertex array.");

            gl.bind_vertex_array(Some(vertex_array));

            // Create a new program.
            let program = gl
                .create_program()
                .expect("Failed to creaate a new program.");

            // Load the shaders.
            let vertex_shader_source = include_str!("tri.vert");
            let fragment_shader_source = include_str!("col.frag");

            info!("Loaded shaders.");

            // Create an array of the shader sources.
            let shader_sources = [
                (glow::VERTEX_SHADER, vertex_shader_source),
                (glow::FRAGMENT_SHADER, fragment_shader_source),
            ];

            let mut shaders: Vec<NativeShader> = Vec::with_capacity(shader_sources.len());

            for (shader_type, shader_source) in shader_sources.iter() {
                let shader = gl
                    .create_shader(*shader_type)
                    .expect("Failed to create a shader");

                // Set the source of the shader.
                gl.shader_source(shader, &format!("{}\n\n{}", shader_version, shader_source));

                gl.compile_shader(shader);

                if !gl.get_shader_compile_status(shader) {
                    panic!("{}", gl.get_shader_info_log(shader));
                }

                // Attach shader
                gl.attach_shader(program, shader);

                // Add the shader to `shaders` vector.
                shaders.push(shader);
            }

            info!("Compiled shaders");

            // Link the program
            gl.link_program(program);

            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }

            // Detach the shaders.
            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }

            gl.use_program(Some(program));
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
        }

        Ok(())
    }
}
