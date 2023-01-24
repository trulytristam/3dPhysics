mod object;
mod GJK;
pub mod input_manager;

extern crate nalgebra;
extern crate glium;
use glium::Surface;
use nalgebra::*;
mod graphic_debug;
mod physics_manager;
use GJK::*;
mod light;
use object::Object;
use physics_manager::*;
use object::BasicShape;
use light::*;
use physics_manager::*;
use graphic_debug::GraphicDebug;
use input_manager::*;
use constraints::ConstraintDesc;


type V3 = Vector3<f64>;
pub struct ObjectManager {
    pub screen_dim: (f64,f64),
    pub cam: (V3,UnitQuaternion<f64>),
    pub cam_speed: f64,
    pub debug: GraphicDebug, 
    objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub physics_manager: PhysicsManager,
    pub input_manager: InputManager,
}

impl ObjectManager {
    pub fn new()-> Self{
        let o: Vec<Object> = vec![];
        let d = GraphicDebug{lines: vec![],line_colors: vec![], dots: vec![]};
        let mut om = ObjectManager{cam_speed: 20.,input_manager: InputManager::new_default(),lights: vec![], physics_manager: PhysicsManager::new(), objects: o,cam: (V3::new(0.,0.,-1.), UnitQuaternion::default()), debug: d, screen_dim: (0.,0.)};
        //shapes
        om.add_object(V3::new(0.0,0.,4.5),BasicShape::Cube([5.,0.3,1.2]), true, 10., V3::new(0.8,0.4,0.)*1.);
        om.add_object(V3::new(0.0,0.,6.5),BasicShape::Cube([5.,0.3,1.2]), false ,20.1, V3::new(-2.0,0.,0.));
        //walls
        om.add_object(V3::new(0.0,-10.,5.),BasicShape::Cube([20.,1.2,20.]), true ,20.1, V3::new(0.0,0.,0.));
        om.add_object(V3::new(0.0,0.,15.),BasicShape::Cube([20.,20.,1.2]), true ,20.1, V3::new(0.0,0.,0.));
        om.add_light(V3::new(10.,10.,-4.),V3::new(1.2,0.4,0.1),0.9);
        om.add_light(V3::new(0.,100.,0.),V3::new(0.1,0.2,0.9),0.5);
        om.add_light(V3::new(-6.,0.,-5.),V3::new(0.4,0.4,0.4),0.2);
        om.cam.0.y -= 2.;
        om.cam.0.z -= 2.;
        let desc = ConstraintDesc{
            apoint: V3::new(-2.5,0.0,0.0),
            bpoint: V3::new(-2.5,0.0,0.0),
            has_distance: true,
            has_angular: true,
            distance_compliance: 0.00000,
            angular_compliance: 0.000000010,
            distance:  0.0, 
            aorient: UnitQuaternion::<f64>::default(),
            borient: UnitQuaternion::<f64>::default(),
            ajoint_axis: (V3::default(),V3::default(),V3::default()),
            bjoint_axis: (V3::default(),V3::default(),V3::default()),
        };
        let desc2 = ConstraintDesc{
            apoint: V3::new(2.5,0.0,0.0),
            bpoint: V3::new(1.5,0.0,0.0),
            has_distance: true,
            has_angular: false,
            distance_compliance: 0.001,
            angular_compliance: 0.00001,
            distance:  2.,
            aorient: UnitQuaternion::<f64>::default(),
            borient: UnitQuaternion::<f64>::default(),
            ajoint_axis: (V3::default(),V3::default(),V3::default()),
            bjoint_axis: (V3::default(),V3::default(),V3::default()),
        };
        
        om.physics_manager.add_constraint(0, 1, desc);
        om.physics_manager.constraints[0].c_desc.generate_default_joint_axis_two();
//        om.physics_manager.add_constraint(0, 1, desc2);
        om
    }
    pub fn handle_input(&mut self, deltatime: f64){
        let cx = self.get_cam_dir_X();
        let cy = self.get_cam_dir_Y();
        let cz = self.get_cam_dir_Z();

        if self.input_manager.get_key_held(Key::W){
            self.cam.0 += cz*self.cam_speed*deltatime; 
        }
        if self.input_manager.get_key_held(Key::S){
            self.cam.0 -= cz*self.cam_speed*deltatime; 
        }
        if self.input_manager.get_key_held(Key::A){
            self.cam.0 -= cx*self.cam_speed*deltatime; 
        }
        if self.input_manager.get_key_held(Key::D){
            self.cam.0 += cx*self.cam_speed*deltatime; 
        }

        if self.input_manager.get_key_held(Key::UP){
            self.cam.1 = UnitQuaternion::from_axis_angle(&UnitVector3::new_normalize(cx), self.cam_speed* -deltatime* 0.15)* self.cam.1;
        }
        if self.input_manager.get_key_held(Key::DOWN){
            self.cam.1 = UnitQuaternion::from_axis_angle(&UnitVector3::new_normalize(cx), self.cam_speed*  deltatime* 0.15)* self.cam.1;
        }
        if self.input_manager.get_key_held(Key::LEFT){
            self.cam.1 = UnitQuaternion::from_axis_angle(&UnitVector3::new_normalize(V3::new(0.,1.,0.)), self.cam_speed* -deltatime* 0.15)* self.cam.1;
        }
        if self.input_manager.get_key_held(Key::RIGHT){
            self.cam.1 = UnitQuaternion::from_axis_angle(&UnitVector3::new_normalize(V3::new(0.,1.,0.)), self.cam_speed*  deltatime* 0.15)* self.cam.1;
        }
    }
    pub fn get_cam_dir_X(&self)-> V3{
        self.cam.1 * V3::new(1.,0.,0.) 
    }
    pub fn get_cam_dir_Y(&self)-> V3{
        self.cam.1 * V3::new(0.,1.,0.)
    }
    pub fn get_cam_dir_Z(&self)-> V3{
        self.cam.1 * V3::new(0.,0.,1.) 
    }
    pub fn add_object(&mut self, p: V3, s: BasicShape, bstatic: bool, d: f64, a: V3){ match s{
            BasicShape::Cube(dim) => self.objects.push(Object::new(p, BasicShape::Cube(dim), bstatic, d,a) ),
            BasicShape::Pyramid => {},
            BasicShape::Sphere(r) => {} ,
        } 
    }
    pub fn add_light_default(&mut self,p: V3){
        let light = Light {pos: p, color: V3::new(0.7,0.5,0.2), brightness: 0.7}; 

        self.lights.push(light);

    }
    pub fn add_light(&mut self,p: V3, c: V3, b: f64){
        let light = Light {pos: p, color: c, brightness: b}; 

        self.lights.push(light);

    }
        
    pub fn get_len(&self) -> f32{
        self.objects.len() as f32
    }
    pub fn update(&mut self, dt: f64, ct: f64, dim: (f64,f64)){
        self.screen_dim = dim;
        self.debug.clear();
        self.handle_input(dt);
        self.input_manager.update();
        self.physics_manager.update_physics(&mut self.objects, dt,ct);

        self.add_debug_lines_for_cube(0, V3::new(1.,1.,0.)); 
        self.add_debug_lines_for_cube(1,V3::new(1.,0.,1.)); 
//        self.lights[0].pos.x = (ct*0.5).sin()*10.; 
//        self.debug.debug_constraint(&self.physics_manager.constraints[0], &self.objects);
//        self.debug.debug_constraint(&self.physics_manager.constraints[1], &self.objects);
    }
    pub fn get_cam_pos(&self)-> [f32;3]{
        return [self.cam.0.x as f32,self.cam.0.y as f32,self.cam.0.z as f32];
    }
    pub fn render(&mut self, display: &glium::Display, program: &glium::Program, debug_p: &glium::Program, dim:(f64,f64), current_time: f64){
        let mut target = display.draw();
        let vertex_buffer = {
            glium::VertexBuffer::new(display, 
                &[
                    Vertex { position: [-1.0,  1.0] },
                    Vertex { position: [-1.0, -1.0] },
                    Vertex { position: [ 1.0,  1.0] },
                    Vertex { position: [ 1.0, -1.0] },
                ]
            ).unwrap()
        };
        // Create an index buffer
        let index_buffer = glium::IndexBuffer::new(display,
            glium::index::PrimitiveType::TrianglesList,
            &[0u16, 1, 2,1,2,3],
        ).unwrap();
        let ub_pos = glium::uniforms::UniformBuffer::new(display,self.get_object_position()).unwrap();
        // println!("frself main: {:?}", OM.getObjectOrientations()[0] );
        let ub_o = glium::uniforms::UniformBuffer::new(display,self.get_object_orientations()).unwrap();
        let debug_line_colors = glium::uniforms::UniformBuffer::new(display,self.debug.get_line_colors() ).unwrap();
        let ub_o_dim = glium::uniforms::UniformBuffer::new(display,self.get_object_dims()).unwrap();
        let lights_buffer = glium::uniforms::UniformBuffer::new(display,self.get_lights()).unwrap();
        let uniforms = glium::uniform! {
            windowSizeX: dim.0 as u32,
            windowSizeY: dim.1 as u32,
            cPos: self.get_cam_pos(),
            iTime: current_time, 
            light_count : self.lights.len() as f32,
            object_count: self.get_len(),
            lineColors: &debug_line_colors,
            lights: &lights_buffer,
            positions: &ub_pos,
            dims: &ub_o_dim,
            orientations: &ub_o};
        target.clear_color(0.,0.,0.,0.);
        let debug_v = self.get_debug_program(&display);

        let param = glium::DrawParameters{
            line_width: Some(3.0),
            ..Default::default()
        };
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);
        target.draw(&vertex_buffer, &index_buffer, &program,&uniforms,&Default::default()).unwrap();
//        target.draw(&debug_v, &indices, &debug_p ,&uniforms,&Default::default()).unwrap();
        target.finish().unwrap();
    }

    fn add_debug_lines_for_cube(&mut self,  id: usize, color: V3) {

            if id >= self.objects.len() {return}
            let out = vec![
                self.objects[id].collider.data[0],
                self.objects[id].collider.data[1],
                self.objects[id].collider.data[3],
                self.objects[id].collider.data[2],
                self.objects[id].collider.data[0],
                self.objects[id].collider.data[4],
                self.objects[id].collider.data[6],
                self.objects[id].collider.data[7],
                self.objects[id].collider.data[5],
                self.objects[id].collider.data[4],
            ];
            self.debug.add_set_of_points_as_connected_lines(out, color); 
            self.debug.addline(self.objects[id].collider.data[2],self.objects[id].collider.data[6], color);
            self.debug.addline(self.objects[id].collider.data[3],self.objects[id].collider.data[7], color);
            self.debug.addline(self.objects[id].collider.data[1],self.objects[id].collider.data[5], color);
    }
    pub fn get_object_position(&self)->[f32;512]{
        let mut vp  = [0.;512]; 
        for oi in 0..self.objects.len(){
            let o = self.cam.1.inverse() * (&self.objects[oi].p- self.cam.0);
            vp[oi*3]= o.x as f32;
            vp[oi*3+1]= o.y as f32;
            vp[oi*3+2]= o.z as f32;
        }   
        vp
    }
    pub fn get_lights(&self)->[f32;512]{
        let mut out = [0.;512];
        let mut b = 0;
        for light in self.lights.iter() {
            
            let a = light.transform_then_to_array(self.cam);
            for i in 0..a.len() {
                out[(b+i)as usize] = a[i];
            }
            b+=a.len();
         }
        out
    }
    pub fn get_object_dims(&self)->[f32;512]{
        let mut vp = [0.;512];
        for oi in 0..self.objects.len(){
            vp[oi*3] = self.objects[oi].dim[0] as f32;
            vp[oi*3+1] = self.objects[oi].dim[1] as f32;
            vp[oi*3+2] = self.objects[oi].dim[2] as f32;
        }
        vp
    }
    pub fn get_object_orientations(&self)->[f32;1024]{
        let mut vpout  = [0.;1024];
        let mut i = 0;
        for o in self.objects.iter(){
            let or =(self.cam.1.inverse() * o.o).to_rotation_matrix();
            for j in 0..3{
                for k in 0..3{
                    vpout[i*9+(j*3+k)]= or[(j,k)] as f32; 
                }
            }
            i+=1;
        }   

        // println!("mat: {:?}", vpout[0]); 
        vpout
    }
    fn get_debug_program(&mut self,display:& glium::Display)->glium::VertexBuffer<VertexLine>{
        let lines = self.debug.getlines(self.screen_dim,self.cam);
        let linecolors = self.debug.get_line_colors();
        // println!("{:?}", linecolors);
        let mut debuglines: Vec<VertexLine> = vec![];
        let mut debug_index_data:Vec<u16> = vec![];
        let mut dim = (display.get_framebuffer_dimensions().0 as f32,display.get_framebuffer_dimensions().0 as f32) ; 
        
        for i in 0..256{
            // debuglines.push(Vertex {position: [lines[i].0,(dim.1/dim.0)*lines[i].1]});  
            let j = i*4;
            debuglines.push(VertexLine {position: [lines[j] as f32, lines[j+1] as f32], color: [linecolors[i*3] as f32,linecolors[i*3+1] as f32,linecolors[i*3+2] as f32]});  
            debuglines.push(VertexLine {position: [lines[j+2] as f32,  lines[j+3] as f32], color: [linecolors[i*3] as f32,linecolors[i*3+1] as f32,linecolors[i*3+2] as f32]});  
            debug_index_data.push(i as u16);
        }
        let debug_vertex_buffer = glium::VertexBuffer::new(display, &debuglines).unwrap(); 
        // let debug_index_buffer = glium::IndexBuffer::new(display,glium::index::PrimitiveType::LinesList,&debug_index_data).unwrap();


        debug_vertex_buffer

    }

}
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
// Create a vertex buffer
glium::implement_vertex!(Vertex,position);

#[derive(Copy, Clone)]
struct VertexLine {
    position: [f32; 2],
    color: [f32;3],
}
// Create a vertex buffer
glium::implement_vertex!(VertexLine,position, color);
