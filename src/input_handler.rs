use log::warn;

#[derive(Default)]
pub(crate) struct InputHandler {
    keys: Vec<Key>,
}

#[derive(PartialEq, Debug)]
pub enum Key {
    W,
    A,
    S,
    D,
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl InputHandler {
    pub(crate) fn add_key(&mut self, key_variant: Key) {
        if !self.keys.contains(&key_variant) {
            self.keys.push(key_variant);
        } else {
            warn!("Key already pressed");
        }
    }

    pub(crate) fn remove_key(&mut self, key_variant: Key) {
        if !self.keys.contains(&key_variant) {
            self.keys.retain(|k| k != &key_variant);
        } else {
            warn!("Key not pressed");
        }
    }

    pub fn is_key_down(&self, key_variant: Key) -> bool {
        self.keys.contains(&key_variant)
    }
}
