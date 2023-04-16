use taconite::*;

fn main() {
    pretty_env_logger::init();

    let mut taconite = Taconite::default();

    taconite.start(WindowConfig {
        name: "Start Example",
        ..Default::default()
    });
}
