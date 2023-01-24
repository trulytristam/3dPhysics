extern crate nalgebra;
use nalgebra::*;

type V3 = Vector3<f64>;

fn line_closest(p: &V3, d: V3, dn: V3) -> V3 {
    let dist = -p.dot(&dn);
    let dlen = d.norm();
    let distc = clamp(dist, 0., dlen);
    let areas = (dist < 0., dist > dlen);
    let a = p;
    let b = p + dlen * dn;
    let pout = p + distc * dn;
    match areas {
        (false, false) => pout ,
        (true, false) => a.clone(),
        (false, true) => b.clone(),
        (true, true) => {
            println!("{:?}", "from Gjk line_closes");
            p.clone()
        }
    }
}
pub fn triangle_closest(i: &V3, j: &V3, k: &V3) -> V3 {
    let ij = j - i;
    let jk = k - j;
    let ki = i - k;

    let ijn = ij.normalize();
    let jkn = jk.normalize();
    let kin = ki.normalize();

    let mut normal = ij.cross(&jk).normalize();
    let ivn = ijn.cross(&normal).normalize();
    let jvn = jkn.cross(&normal).normalize();
    let kvn = kin.cross(&normal).normalize();

    let kdist = jk.dot(&ivn);
    let area = ij.norm() * kdist / 2.;

    let mut a = -ivn.dot(&i);
    let mut b = -jvn.dot(&j);
    let mut c = -kvn.dot(&k);

    let mut u = a * ij.norm() / 2. / area;
    let mut v = b * jk.norm() / 2. / area;
    let mut w = 1. - u - v;

    let areas = (a < 0., b < 0., c < 0.);

    match areas {
        (true, true, true) => v * i + w * j + u * k,
        (true, true, false) => line_closest(k, ki, kin),
        (true, false, false) => k.clone(),
        (true, false, true) => line_closest(j, jk, jkn),
        (false, true, true) => line_closest(i, ij, ijn),
        (false, true, false) => i.clone(),
        (false, false, false) => j.clone(),
        (false, false, true) => j.clone(),
    }
}

pub trait IntersectsRay {
    // add code here
    fn intersect_ray(&self, ro: V3, rd: V3)->Option<V3>{
        None
    }
}

impl IntersectsRay for (V3,V3){//plane interseciton
    fn intersect_ray(&self, ro: V3, rd: V3)->Option<V3>{
        let nn = -self.1;
        let d = (ro-self.0).dot(&nn);
        let rp = rd.dot(&nn);
        let ratio = 1./rp;
        if d > 0. && rp > 0. {Some(ro + rd * ratio * d)} else{None} 
    }
}

impl IntersectsRay for (V3,V3,V3){//triangle intersection
    fn intersect_ray(&self, ro: V3, rd: V3)-> Option<V3>{
        let ab = self.1 - self.0;
        let ac = self.2 - self.0;
        let plane_intersect = (self.0, ab.cross(&ac)).intersect_ray(ro,rd);
        if let Some(point) = plane_intersect {
            Some(
                point + triangle_closest(&(self.0-point), &(self.1-point), &(self.1-point))
            )
        }
        else{
            None
        }

    }
}
                                  
