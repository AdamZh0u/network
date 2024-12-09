use rand::Rng;

pub struct SpinAgent {
    pub spin: i8,
}

impl SpinAgent {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            spin: if rng.gen_bool(0.5) { 1 } else { -1 },
        }
    }

    pub fn flip(&mut self) {
        self.spin = -self.spin;
    }
} 