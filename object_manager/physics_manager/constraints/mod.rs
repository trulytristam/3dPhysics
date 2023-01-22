
use super::super::object::*;
extern crate nalgebra;
use nalgebra::*;
type V3 = Vector3<f64>; 
type UQ = UnitQuaternion<f64>;
type M3 = Matrix3<f64>;
use std::f64::consts::PI;

enum ConstraintType{
}

pub struct ConstraintDesc {
    pub apoint: V3,
    pub bpoint: V3,
    pub has_distance: bool, 
    pub distance_compliance: f64,
    pub distance: f64,

    pub has_angular: bool,
    pub angular_compliance: f64,
    pub aorient: UQ,
    pub borient: UQ,

    pub ajoint_axis: (V3,V3,V3),
    pub bjoint_axis: (V3,V3,V3),
    
}

impl ConstraintDesc {
    fn get_correction(&self, a: &mut Object, b: &mut Object)->Vector3<f64>{

//        let t2 = UnitQuaternion::new(V3::new(PI/2.,0.,0.));
//        let diff_current_ab = (a.o*UnitQuaternion::new(V3::new(0.,PI,0.))*t2) * b.o.inverse();
//        2.* V3::new(diff_current_ab.i,diff_current_ab.j,diff_current_ab.k)
        return (a.o*self.ajoint_axis.2).cross(&(b.o*self.bjoint_axis.2)); 
          
    }

    pub fn generate_default_joint_axis_two(&mut self){
        self.ajoint_axis = ConstraintDesc::generate_default_joint_axis_one(self.apoint);
        self.bjoint_axis = ConstraintDesc::generate_default_joint_axis_one(self.bpoint);
    }
    fn generate_default_joint_axis_one(r:V3)->(V3,V3,V3){
        let a = r.normalize();
        let b = V3::new(0.,1.,0.).cross(&r).normalize();
        let c = a.cross(&b);

        (a,c,b)
    }

}
pub struct Constraint{
    pub a: u32,
    pub b: u32,
    pub c_desc: ConstraintDesc,
    pub lagrange: f64,
    pub angular_lagrange: f64,
}

impl Constraint {
    fn solve_constraing_angular(&mut self, o1: &mut Object, o2: &mut Object, h: f64){
        let nvec = self.c_desc.get_correction(o1,o2); 
        if nvec.norm() <=0.00001 {return;}
        
        let n = nvec.normalize(); 
        let n1_local = (o1.o.inverse() * n); 
        let n2_local = (o2.o.inverse() * n); 

        let w1 = (n1_local.transpose()* o1.ii_t* n1_local).x; 
        let w2 = (n2_local.transpose()* o2.ii_t* n2_local).x; 
       
        let c = nvec.norm();
        let a = self.c_desc.angular_compliance / (h*h);

        let top = -c-a* self.angular_lagrange;
        let bot = w1 + w2 + a;
        let dy =  top/bot;
        self.angular_lagrange += dy;
                        
        let p = n*dy;
        
        //update pos
        let p1_local = o1.o.inverse()*p;
        let p2_local = o2.o.inverse()*p;
        let mut aq = o1.ii_t * p1_local;
        let mut bq = o2.ii_t * p2_local;
        
        
        aq = o1.o * aq;
        bq = o2.o * bq;
        let o1on = o1.o.normalize();
        let o2on = o2.o.normalize();
        
        o1.o = UQ::new_normalize(o1on + 0.5* Quaternion::<f64>::new(0.,aq.x,aq.y,aq.z)*o1on); 
        o2.o = UQ::new_normalize(o2on - 0.5* Quaternion::<f64>::new(0.,bq.x,bq.y,bq.z)*o2on); 

    }
    fn solve_constraint_linear(&mut self, o1: &mut Object, o2: &mut Object, h: f64){
        


        let r1_global = o1.localtoglobal(self.c_desc.apoint);
        let r2_global = o2.localtoglobal(self.c_desc.bpoint);
        

        let ii_ta = o1.ii_t;
        let ii_tb = o2.ii_t;
        let between = r2_global-r1_global;
        let n = between.normalize();
        let r1 = r1_global - o1.p; 
        let r2 = r2_global - o2.p;
        let c = self.c_desc.distance - between.norm();

        let r1n1 = o1.o.inverse()*r1.cross(&n);
        let r2n2 = o2.o.inverse()*r2.cross(&n);
        let w1 = (1./o1.m) + (r1n1.transpose()*ii_ta*r1n1).x; 
        let w2 = (1./o2.m) + (r2n2.transpose()*ii_tb*r2n2).x; 
        
        let a = self.c_desc.distance_compliance / (h*h);
        let dy = (-c-a* self.lagrange) /
                        (w1 + w2 + a);
        let p = n*dy;
        
        //update pos
        o1.p += p/o1.m;
        o2.p -= p/o2.m;

        let p1_local = o1.o.inverse()*p;
        let p2_local = o2.o.inverse()*p;
        let mut aq = o1.ii_t * (self.c_desc.apoint.cross(&p1_local));
        let mut bq = o2.ii_t * (self.c_desc.bpoint.cross(&p2_local));
        
        aq = o1.o * aq;
        bq = o2.o * bq;

        let o1on = o1.o.normalize();
        let o2on = o2.o.normalize();
        
        o1.o = UQ::new_normalize(o1on + 0.5* Quaternion::<f64>::new(0.,aq.x,aq.y,aq.z)*o1on); 
        o2.o = UQ::new_normalize(o2on - 0.5* Quaternion::<f64>::new(0.,bq.x,bq.y,bq.z)*o2on); 


        self.lagrange += dy;
        
    }

    pub fn solve_constraint(&mut self, objects: &mut Vec<Object>, h:f64){
        let mid = if self.a > self.b {self.a}else{self.b};
        let bot = if self.a < self.b {self.a}else{self.b};
        let (left,right) = objects.split_at_mut(mid as usize);
        if self.c_desc.has_distance {
            self.solve_constraint_linear(&mut left[bot as usize],&mut right[0], h);
        }

        if self.c_desc.has_angular {
            self.solve_constraing_angular(&mut left[bot as usize],&mut right[0], h);
        }
    }
    pub fn initialize(&mut self){
        self.lagrange = 0.;
        self.angular_lagrange = 0.;
    }
}
pub struct PhysicsManager {
    pub constraints: Vec<Constraint>,
}


