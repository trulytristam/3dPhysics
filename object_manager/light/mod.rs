extern crate nalgebra;
use nalgebra::*;
type V3 = Vector3<f64>;

pub struct Light {
    pub pos: V3,
    pub color: V3,
    pub brightness: f64,
}

impl Light {
    pub fn transform_then_to_array(&self, cam: (V3,UnitQuaternion<f64>) )->[f32;7]{
        let transormed_pos = cam.1.inverse()*(self.pos-cam.0);
        let mut out = [0.;7];
        out[0] = transormed_pos.x as f32; 
        out[1] = transormed_pos.y as f32;
        out[2] = transormed_pos.z as f32; 
 
        out[3] = self.color.x as f32; 
        out[4] = self.color.y as f32; 
        out[5] = self.color.z as f32; 
        out[6] = self.brightness as f32; 
        out
    }
}
