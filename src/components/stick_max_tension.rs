/// The stick will break when its size becomes bigger than its `length` multiplied by this factor
///
/// If you set it to `1.0` the stick will break almost instantly
/// If you set it to `2.0` the stick will break when stretched to twice its `length`
#[derive(Debug, Clone)]
pub struct VerletStickMaxTension(pub f32);
