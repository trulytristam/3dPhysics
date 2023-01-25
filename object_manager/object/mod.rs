extern crate nalgebra;
extern crate rand;

use rand::Rng;
#[allow(dead_code)]
pub enum BasicShape {
    Cube([f64; 3]),
    Pyramid,
    Sphere(f64),
}
use nalgebra::*;

use super::{GJK::Collider, physics_manager::ray_shape_intersection::IntersectsRay};
type V3 = Vector3<f64>;
type V4 = Vector4<f64>;

#[allow(dead_code)]
#[derive(Clone,Default)]
pub struct Object {
    pub p: V3,
    pub old_p: V3,
    v: V3,
    pub o: UnitQuaternion<f64>,
    pub old_o: UnitQuaternion<f64>,
    pub is_static: bool,
    a: V3,
    pub dim: [f64; 3],
    pub data: Vec<V3>,
    pub collider: Collider,
    pub trianglelist: Vec<(usize,usize,usize)>,
    f_ext: V3,
    t_ext: V3,

    pub m: f64,
    i_m: f64,
    inertia_tensor_local: Matrix3<f64>,
    i_t: Matrix3<f64>,
    pub ii_t: Matrix3<f64>,
    pub oriented_ii_t: Matrix3<f64>,
}
fn create_default_object() -> Object {
    Object {
        is_static: true,
        m: 20.,
        oriented_ii_t: Matrix3::default(),
        inertia_tensor_local: Matrix3::default(),
        t_ext: V3::default(),
        f_ext: V3::default(),
        old_p: V3::new(0., 0., 0.),
        p: V3::new(0., 0., 0.),
        collider: Collider { data: vec![] },
        dim: [1., 1., 1.],
        v: V3::new(0., 0., 0.),
        old_o: UnitQuaternion::default(),
        o: UnitQuaternion::default(),
        a: Vector3::<f64>::new(0., 0., 0.),
        data: vec![],
        i_m: 0.,
        ii_t: Matrix3::default(),
        i_t: Matrix3::default(),
        trianglelist: vec![],
    }
}
fn quaternion_to_rotation_matrix(q: UnitQuaternion<f64>) {}
impl Object {
    pub fn generate_rectangle_tensor(w: f64, h: f64, d: f64, mass: f64) -> Matrix3<f64> {
        let w2 = w * w;
        let h2 = h * h;
        let d2 = d * d;

        let mo12 = mass / 12.;
        let x = (w2 + h2) * mo12;
        let y = (d2 + h2) * mo12;
        let z = (w2 + d2) * mo12;
        let m = Matrix3::<f64>::new(x, 0., 0., 0., y, 0., 0., 0., z);
        return m;
    }
    pub fn orient_ii_t(&mut self) {
        self.oriented_ii_t = self.ii_t * self.o.to_rotation_matrix();
    }
    pub fn new(pp: V3, ss: BasicShape, bstatic: bool, dd: f64, aa: V3) -> Object {
        let mut temp = create_default_object();
        match ss {
            BasicShape::Cube(dim) => {
                temp.p = pp;
                temp.m = dim[0] * dim[1] * dim[2] * dd;
                if bstatic {
                    temp.m = 100000000000.; //100000000000.0;
                }
                let it = Object::generate_rectangle_tensor(dim[0], dim[1], dim[2], temp.m);
                temp.inertia_tensor_local = it;
//                let rng = rand::thread_rng();
//                let mut rd = || rng.gen_range(-1.0f64..1.0);
                temp.is_static = bstatic;
                temp.a = aa;
                // temp.a = Vector4::new(0.,1.,0.,0.4).normalize();
                temp.dim = dim;
                let dir: [f64; 2] = [0.5, -0.5];
                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            temp.data.push(V3::new(
                                dir[i] * dim[0],
                                dir[j] * dim[1],
                                dir[k] * dim[2],
                            ))
                        }
                    }
                }
                println!("temp data from object new: {:?}",temp.data); 
                temp.trianglelist.push((0,2,3)); // right
                temp.trianglelist.push((3,1,0));
                temp.trianglelist.push((0,1,5)); // top
                temp.trianglelist.push((5,4,0));
                temp.trianglelist.push((1,3,7)); // front
                temp.trianglelist.push((7,5,1));
                temp.trianglelist.push((4,5,7)); // left
                temp.trianglelist.push((7,6,4));
                temp.trianglelist.push((7,3,2)); // bot
                temp.trianglelist.push((2,6,7));
                temp.trianglelist.push((0,4,6)); // back
                temp.trianglelist.push((6,2,0));
            }
            BasicShape::Pyramid => {}
            BasicShape::Sphere(r) => {}
        };
        temp
    }
    ///find intersection with ray in world space and return an Option<V3> of the local intersection
    ///output is point and distance
    pub fn intersects(&self, ro: V3, rd: V3)->Option<(V3,f64)>{
        let mut  output: Option<V3> = None;
        for tl in self.trianglelist.iter(){
            let ct = self.get_collider_triangle(tl);
            if output == None{
                output = ct.intersect_ray(ro, rd);
            }//output is global
        }        
        if output == None {
            return None;
        }
        else {
            let global_point = self.globaltolocal(output.unwrap()); 
            return Some((global_point,(output.unwrap()-ro).dot(&rd)));
        }
    }
    pub fn get_collider_triangle(&self, tl: &(usize,usize,usize))->(V3,V3,V3){
        let c = &self.collider;
        (c.data[tl.0],
         c.data[tl.1],
         c.data[tl.2])
    }
    pub fn generate_collider(&mut self) {
        self.collider.data.clear();
        for p in self.data.iter() {
            self.collider.data.push(self.localtoglobal(p.clone()));
        }
        self.i_t = self.inertia_tensor_local; // o.to_rotation_matrix();
        let iitopt = self.i_t.try_inverse();
        self.ii_t = iitopt.unwrap();
    }
    pub fn update(&mut self, h: f64) {
        self.old_p = self.p;
        let grav = if self.is_static { 0. } else { -69. };
        self.v += (self.f_ext * self.i_m * h).xyz() + V3::new(0., grav, 0.) * h;
        self.p += self.v * h;
        self.old_o = self.o;

        let inva = self.o.inverse() * self.a;
        let tra = self.i_t * inva;
        let rhs = (self.o.inverse() * self.t_ext - (inva.cross(&tra)));
        self.a += self.o * (self.ii_t * rhs * h);

        let q1 = self.o.normalize();
        let q2 = Quaternion::<f64>::new(0., self.a.x, self.a.y, self.a.z) * 0.5 * h * q1;
        self.o = UnitQuaternion::new_normalize(q1 + q2);
    }
    pub fn update_velocities(&mut self, h: f64) {
        let damping = if self.is_static { 0. } else { 0.41 };
        self.v = (self.p - self.old_p) / h;
        self.v = self.v - (self.v * damping * h);
        let dq1 = self.o * self.old_o.inverse();
        let dq = dq1.normalize();
        let length = self.a.norm();
        self.a = 2. * V3::new(dq.i, dq.j, dq.k) / h;
        self.a = if dq.w >= 0. { self.a } else { -self.a };
        self.a = self.a - (self.a * damping * h);
        //        self.a = self.a.normalize() * length;// - (length*h*0.10));
    }

    pub fn localtoglobal(&self, p: V3) -> V3 {
        self.o * p + self.p
    }
    pub fn globaltolocal(&self, p: V3) -> V3 {
        self.o.inverse() * (p - self.p)
    }
}
