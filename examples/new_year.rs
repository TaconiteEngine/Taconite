use taconite::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    taconite::init_logging();

    NewYear::one();

    Ok(())
}
