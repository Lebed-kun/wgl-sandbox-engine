// TODO: replace 'f32's with this alias
pub type Float = f32;
pub type Vector4 = [Float; 4];
pub type Matrix4x4 = [Float; 4 * 4];

pub struct XYTuple {
    pub x: Float,
    pub y: Float
}

pub struct DrawProps {
    pub position: XYTuple,
    pub rotation: Float,
    pub scale: XYTuple,
    pub color: Vector4,
}
