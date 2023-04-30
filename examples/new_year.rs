use taconite::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = tracing_subscriber::fmt().compact().without_time().finish();
    tracing::subscriber::set_global_default(subscriber)?;

    NewYear::one();

    Ok(())
}
