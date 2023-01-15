use taconite::*;

fn main() {
    env_logger::init();

    let mut taconite = Taconite::default();

    taconite.start(WindowConfig {
        name: "Window example",
        vsync: true,
        width: 800,
        height: 600,
    });
}
