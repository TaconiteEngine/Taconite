use taconite::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    taconite::init_logging();

    let mut taconite = Taconite::default();

    taconite.start(WindowConfig {
        name: "Start Example".into(),
        ..Default::default()
    });

    Ok(())
}
