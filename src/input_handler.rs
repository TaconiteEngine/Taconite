#[derive(Default)]
pub(crate) struct InputHandler {
    pub keys: Vec<Key>,
}

pub enum Key {
    W,
    A,
    S,
    D,
}
