pub struct Entity {
    alive: bool,
}

impl Default for Entity {
    fn default() -> Self {
        Self { alive: true }
    }
}

impl Entity {
    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn enable(&mut self) {
        self.alive = true;
    }

    pub fn disable(&mut self) {
        self.alive = false;
    }
}
