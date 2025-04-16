use nalgebra::Matrix4;
use nalgebra::Vector3;
use nalgebra::Quaternion;

pub struct Transform {
    mat: Matrix4<f32>,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            mat: Matrix4::identity(),
        }
    }

    pub fn print(&mut self) {
        //imprime a matriz 4x4
    }

    pub fn scale_by(&mut self, scalar: f32) {
        //multiplica o objeto por um escalar
    }
    
    pub fn scale_by_axis(&mut self, x: f32, y: f32, z: f32) {
        //multiplica o objeto por um escalar em cada eixo
    }

    pub fn set_scale(&mut self, scalar: f32) {
        //define a escalar do objeto
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        //teleporta para a posição x, y, z
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        //move x, y, z
    }

    pub fn translate_with_vector(&mut self, vector: Vector3<f32>) {
        //move x, y, z
    }

    pub fn rotate(&mut self, quaternion: Quaternion<f32>) {
        //rotaciona o objeto em torno do eixo x, y, z
    }

    pub fn rotate_x(&mut self, angle: f32) {
        //rotaciona o objeto em torno do eixo x
    }
    
    pub fn rotate_y(&mut self, angle: f32) {
        //rotaciona o objeto em torno do eixo y
    }
    
    pub fn rotate_z(&mut self, angle: f32) {
        //rotaciona o objeto em torno do eixo z
    }

    pub fn get_forward_direction(&mut self) {
        //retorna o vetor forward do objeto
    }

    pub fn get_up_direction(&mut self) {
        //retorna o vetor up do objeto
    }

    pub fn get_right_direction(&mut self) {
        //retorna o vetor right do objeto
    }

    pub fn get_position(&mut self) {
        //retorna a posição do objeto
    }

    pub fn get_scale(&mut self) {
        //retorna a escala do objeto
    }

    pub fn reset(&mut self) {
        //reseta a matriz
    }
}