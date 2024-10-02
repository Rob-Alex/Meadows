#[derive(Debug)]
pub struct Material {
    epsilon_r: f32,
    mu_r: f32,
}

impl Material {
    pub fn new(epsilon_r: f32, mu_r: f32) -> Self {
        Self { epsilon_r, mu_r }
    }
}
