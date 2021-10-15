#[derive(Debug, Clone)]
pub enum VerletTimeStep {
    DeltaTime,
    FixedDeltaTime(f64),
}
