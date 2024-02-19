use bytemuck::{Pod, Zeroable};
use encase::impl_vector;

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl AsRef<[f32; 3]> for Color {
    fn as_ref(&self) -> &[f32; 3] {
        bytemuck::cast_ref(self)
    }
}

impl AsMut<[f32; 3]> for Color {
    fn as_mut(&mut self) -> &mut [f32; 3] {
        bytemuck::cast_mut(self)
    }
}

impl From<[f32; 3]> for Color {
    fn from(value: [f32; 3]) -> Self {
        bytemuck::cast(value)
    }
}

impl From<Color> for [f32; 3] {
    fn from(value: Color) -> [f32; 3] {
        bytemuck::cast(value)
    }
}

impl_vector!(3, Color, f32; using AsRef AsMut From);
