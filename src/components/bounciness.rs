#[derive(Debug, Copy, Clone)]
pub struct Bounciness(pub f32);

impl Default for Bounciness {
    fn default() -> Self {
        Self(0.1)
    }
}
