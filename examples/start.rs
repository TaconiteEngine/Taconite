use taconite::*;

fn main() {
    pretty_env_logger::init();

    let mut taconite = Taconite::default();

    taconite.start(WindowConfig {
        name: "Start Example",
        fullscreen: false,
        vsync: true,
        width: 800,
        height: 600,
    });
}
