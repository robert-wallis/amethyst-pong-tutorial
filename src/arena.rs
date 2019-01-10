use std::default::Default;

pub struct Arena {
    pub width: f32,
    pub height: f32,
}

impl Default for Arena {
    fn default() -> Self {
        Arena {
            width: 100.0,
            height: 100.0,
        }
    }
}
