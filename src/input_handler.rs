use log::warn;

#[derive(Default)]
pub(crate) struct InputHandler {
    keys: Vec<Key>,
}

#[allow(dead_code)] // TODO: Will be gone soon
#[derive(PartialEq)]
pub enum Key {
    W,
    A,
    S,
    D,
}

#[allow(dead_code)] // TODO: Will be gone soon
impl InputHandler {
    pub(crate) fn add_key(&mut self, key_variant: Key) {
        if !self.keys.contains(&key_variant) {
            self.keys.push(key_variant);
        } else {
            warn!("Key already pressed");
        }
    }

    pub fn is_key_down(&self, key_variant: Key) -> bool {
        self.keys.contains(&key_variant)
    }
}
