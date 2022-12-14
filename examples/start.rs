use taconite::*;

fn main() {
    let mut taconite = Taconite::default();

    loop {
        taconite.update();
        taconite.draw();
    }
}
