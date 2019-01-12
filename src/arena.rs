pub struct Arena {
    pub width: f32,
    pub height: f32,
}

impl Arena {
    pub fn new(width: f32, height: f32) -> Arena {
        Arena { width, height }
    }
}
