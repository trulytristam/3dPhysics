use super::object::*;
extern crate nalgebra;
use nalgebra::*;
type V3 = Vector3<f64>; 
type UQ = UnitQuaternion<f64>;
pub mod constraints;
use constraints::*;


pub struct PhysicsManager {
    pub constraints: Vec<Constraint>,
}


impl PhysicsManager {
    pub fn new()-> Self{
        PhysicsManager { constraints: vec![]}
    }
    pub fn update_physics(&mut self, objects: &mut Vec<Object>, dt: f64,ct: f64){
        let n_sub = 20;
        let h = dt/(n_sub as f64);
        self.init_contraints();
        for _ in 0..n_sub{
            for o in objects.iter_mut(){
                o.generate_collider();
                o.update(h);
                o.orient_ii_t();
            }        

            self.solve_positions(objects,h);
            for o in objects.iter_mut(){
                if !o.is_static {
                    o.update_velocities(h);
                }
            }
        }
    }
    fn init_contraints(&mut self){
        for c in self.constraints.iter_mut(){
            c.initialize();
        }
    }
    pub fn add_constraint(&mut self, ai:u32, bi:u32, desc: ConstraintDesc){
        let c = Constraint {a:ai, b:bi, c_desc: desc, lagrange: 0., angular_lagrange: 0.};
        self.constraints.push(c);
    }
    fn solve_positions(&mut self, objects: &mut Vec<Object>,h: f64){
        for c in self.constraints.iter_mut() {
            c.solve_constraint(objects, h);
        }
    }
}
