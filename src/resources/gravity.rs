#[derive(Debug, Copy, Clone)]
pub struct VerletGravity(pub f32);

impl Default for VerletGravity {
    fn default() -> Self {
        Self(-9.81)
    }
}
