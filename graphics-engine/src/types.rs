pub type Vector4 = [f32; 4];
pub type Matrix4x4 = [f32; 4 * 4];

pub struct XYTuple {
    pub x: f32,
    pub y: f32
}

pub struct DrawProps {
    pub position: XYTuple,
    pub rotation: f32,
    pub scale: XYTuple,
    pub color: Vector4,
}
