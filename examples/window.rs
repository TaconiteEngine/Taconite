use taconite::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = tracing_subscriber::fmt().compact().without_time().finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let mut taconite = Taconite::default();

    taconite.start(WindowConfig {
        name: "Window example".into(),
        ..Default::default()
    });

    Ok(())
}
