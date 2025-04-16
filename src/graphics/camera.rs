use super::transform::Transform;
use super::projection::Projection;
use nalgebra::Matrix4;

pub struct Camera {
    transofrm: Transform,
    projection: Projection,
    view: Matrix4<f32>,
}

impl Camera {

}