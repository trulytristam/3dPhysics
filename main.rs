extern crate glium;
extern crate nalgebra;
mod object_manager;
mod profile;
use nalgebra::Matrix3;
use nalgebra::Vector2;
use nalgebra::Vector3;
use object_manager::input_manager;
use profile::Profile;


fn main() {
    let init_time = std::time::Instant::now();
    //use glium::{glutin, Surface};
    let mut om = object_manager::ObjectManager::new();
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_min_inner_size(PhysicalSize {
        width: 1000,
        height: 400,
    });
    let cb = glutin::ContextBuilder::new();
    // Create a window
    let display: glium::Display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let fragment_source = std::fs::read_to_string("fragment_shader.glsl").unwrap();
    let vertex_source = std::fs::read_to_string("vertex_shader.glsl").unwrap();
    let debug_frag = std::fs::read_to_string("debugFrag.glsl").unwrap();
    let debug_vert = std::fs::read_to_string("debugVert.glsl").unwrap();
    let debug_program =
        glium::Program::from_source(&display, &debug_vert, &debug_frag, None).unwrap();
    let program =
        glium::Program::from_source(&display, &vertex_source, &fragment_source, None).unwrap();
    // Main loop
    let mut last_time = std::time::Instant::now();
    let mut currenttime = std::time::Instant::now() + std::time::Duration::from_millis(16);
    let event_loop_proxy = event_loop.create_proxy();

    struct MyEvent {}
    std::thread::spawn(move || {
        let mut timer = std::time::Instant::now();
        let timer_dur = 0.016f64;
        loop {
            let different_in_time =
                (std::time::Instant::now() - timer).as_nanos() as f64 / 1_000_000_000.;
            if different_in_time > timer_dur {
                event_loop_proxy.send_event(()).unwrap();
                timer = std::time::Instant::now()
                    + std::time::Duration::from_secs_f64(different_in_time - timer_dur);
            }
        }
    });
    event_loop.run(move |ev, _, control_flow| {
        let deltatime = (currenttime - last_time).as_nanos() as f64 / 1000000000.;
        match ev {
            glutin::event::Event::UserEvent(()) => {
                let ws = display.get_framebuffer_dimensions();
                let current_time = (currenttime - init_time).as_nanos() as f64 / 1000000000.;
                if deltatime > 0.00000001 {
                    let profile = Profile::new("Profile-> OM update time");
                    om.update(0.01666667, current_time, (ws.0 as f64, ws.1 as f64));
                }
                let profile = Profile::new("Profile-> OM render time");
                om.render(
                    &display,
                    &program,
                    &debug_program,
                    (ws.0 as f64, ws.1 as f64),
                    current_time,
                );
            }
            glutin::event::Event::WindowEvent { event, .. } => {
                process_keyboard_input(&mut om, &event, deltatime);
                match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }

                    _ => {
                        return;
                    }
                }
            }
            glutin::event::Event::DeviceEvent { event, .. } => {
                process_mouse_input(&mut om, &event, deltatime);
            }

            _ => (),
        }
        last_time = currenttime;
        currenttime = std::time::Instant::now();
    });
}

use glium::glutin;
use winit::dpi::PhysicalSize;
use winit::window::Fullscreen;
fn process_keyboard_input(
    om: &mut object_manager::ObjectManager,
    event: &glutin::event::WindowEvent<'_>,
    deltatime: f64,
) {
    match *event {
        glutin::event::WindowEvent::KeyboardInput { input, .. } => {
            let pressed = input.state == glutin::event::ElementState::Pressed;
            let key = match input.virtual_keycode {
                Some(key) => key,
                None => return,
            };
            let speed = 200.5f64;
            let aspeed = 50.5f64;
            let forward = om.cam.1 * Vector3::<f64>::new(0., 0., 1.);
            let right = om.cam.1 * Vector3::<f64>::new(1., 0., 0.);
            if pressed {
                match key {
                    glutin::event::VirtualKeyCode::A => {
                        om.input_manager.press_key(input_manager::Key::A)
                    }
                    glutin::event::VirtualKeyCode::D => {
                        om.input_manager.press_key(input_manager::Key::D)
                    }
                    glutin::event::VirtualKeyCode::W => {
                        om.input_manager.press_key(input_manager::Key::W)
                    }
                    glutin::event::VirtualKeyCode::S => {
                        om.input_manager.press_key(input_manager::Key::S)
                    }
                    glutin::event::VirtualKeyCode::B => {
                        om.input_manager.press_key(input_manager::Key::B)
                    }
                    glutin::event::VirtualKeyCode::C => {
                        om.input_manager.press_key(input_manager::Key::C)
                    }
                    glutin::event::VirtualKeyCode::E => {
                        om.input_manager.press_key(input_manager::Key::E)
                    }
                    glutin::event::VirtualKeyCode::F => {
                        om.input_manager.press_key(input_manager::Key::F)
                    }
                    glutin::event::VirtualKeyCode::G => {
                        om.input_manager.press_key(input_manager::Key::G)
                    }
                    glutin::event::VirtualKeyCode::H => {
                        om.input_manager.press_key(input_manager::Key::H)
                    }
                    glutin::event::VirtualKeyCode::I => {
                        om.input_manager.press_key(input_manager::Key::I)
                    }
                    glutin::event::VirtualKeyCode::J => {
                        om.input_manager.press_key(input_manager::Key::J)
                    }
                    glutin::event::VirtualKeyCode::K => {
                        om.input_manager.press_key(input_manager::Key::K)
                    }
                    glutin::event::VirtualKeyCode::L => {
                        om.input_manager.press_key(input_manager::Key::L)
                    }
                    glutin::event::VirtualKeyCode::M => {
                        om.input_manager.press_key(input_manager::Key::M)
                    }
                    glutin::event::VirtualKeyCode::N => {
                        om.input_manager.press_key(input_manager::Key::N)
                    }
                    glutin::event::VirtualKeyCode::O => {
                        om.input_manager.press_key(input_manager::Key::O)
                    }
                    glutin::event::VirtualKeyCode::P => {
                        om.input_manager.press_key(input_manager::Key::P)
                    }
                    glutin::event::VirtualKeyCode::Q => {
                        om.input_manager.press_key(input_manager::Key::Q)
                    }
                    glutin::event::VirtualKeyCode::R => {
                        om.input_manager.press_key(input_manager::Key::R)
                    }
                    glutin::event::VirtualKeyCode::T => {
                        om.input_manager.press_key(input_manager::Key::T)
                    }
                    glutin::event::VirtualKeyCode::U => {
                        om.input_manager.press_key(input_manager::Key::U)
                    }
                    glutin::event::VirtualKeyCode::V => {
                        om.input_manager.press_key(input_manager::Key::V)
                    }
                    glutin::event::VirtualKeyCode::X => {
                        om.input_manager.press_key(input_manager::Key::X)
                    }
                    glutin::event::VirtualKeyCode::Y => {
                        om.input_manager.press_key(input_manager::Key::Y)
                    }
                    glutin::event::VirtualKeyCode::Z => {
                        om.input_manager.press_key(input_manager::Key::Z)
                    }

                    glutin::event::VirtualKeyCode::Left => {
                        om.input_manager.press_key(input_manager::Key::LEFT)
                    }
                    glutin::event::VirtualKeyCode::Right => {
                        om.input_manager.press_key(input_manager::Key::RIGHT)
                    }
                    glutin::event::VirtualKeyCode::Up => {
                        om.input_manager.press_key(input_manager::Key::UP)
                    }
                    glutin::event::VirtualKeyCode::Down => {
                        om.input_manager.press_key(input_manager::Key::DOWN)
                    }
                    _ => (),
                };
            } else {
                match key {
                    glutin::event::VirtualKeyCode::A => {
                        om.input_manager.release_key(input_manager::Key::A)
                    }
                    glutin::event::VirtualKeyCode::D => {
                        om.input_manager.release_key(input_manager::Key::D)
                    }
                    glutin::event::VirtualKeyCode::W => {
                        om.input_manager.release_key(input_manager::Key::W)
                    }
                    glutin::event::VirtualKeyCode::S => {
                        om.input_manager.release_key(input_manager::Key::S)
                    }
                    glutin::event::VirtualKeyCode::B => {
                        om.input_manager.release_key(input_manager::Key::B)
                    }
                    glutin::event::VirtualKeyCode::C => {
                        om.input_manager.release_key(input_manager::Key::C)
                    }
                    glutin::event::VirtualKeyCode::E => {
                        om.input_manager.release_key(input_manager::Key::E)
                    }
                    glutin::event::VirtualKeyCode::F => {
                        om.input_manager.release_key(input_manager::Key::F)
                    }
                    glutin::event::VirtualKeyCode::G => {
                        om.input_manager.release_key(input_manager::Key::G)
                    }
                    glutin::event::VirtualKeyCode::H => {
                        om.input_manager.release_key(input_manager::Key::H)
                    }
                    glutin::event::VirtualKeyCode::I => {
                        om.input_manager.release_key(input_manager::Key::I)
                    }
                    glutin::event::VirtualKeyCode::J => {
                        om.input_manager.release_key(input_manager::Key::J)
                    }
                    glutin::event::VirtualKeyCode::K => {
                        om.input_manager.release_key(input_manager::Key::K)
                    }
                    glutin::event::VirtualKeyCode::L => {
                        om.input_manager.release_key(input_manager::Key::L)
                    }
                    glutin::event::VirtualKeyCode::M => {
                        om.input_manager.release_key(input_manager::Key::M)
                    }
                    glutin::event::VirtualKeyCode::N => {
                        om.input_manager.release_key(input_manager::Key::N)
                    }
                    glutin::event::VirtualKeyCode::O => {
                        om.input_manager.release_key(input_manager::Key::O)
                    }
                    glutin::event::VirtualKeyCode::P => {
                        om.input_manager.release_key(input_manager::Key::P)
                    }
                    glutin::event::VirtualKeyCode::Q => {
                        om.input_manager.release_key(input_manager::Key::Q)
                    }
                    glutin::event::VirtualKeyCode::R => {
                        om.input_manager.release_key(input_manager::Key::R)
                    }
                    glutin::event::VirtualKeyCode::T => {
                        om.input_manager.release_key(input_manager::Key::T)
                    }
                    glutin::event::VirtualKeyCode::U => {
                        om.input_manager.release_key(input_manager::Key::U)
                    }
                    glutin::event::VirtualKeyCode::V => {
                        om.input_manager.release_key(input_manager::Key::V)
                    }
                    glutin::event::VirtualKeyCode::X => {
                        om.input_manager.release_key(input_manager::Key::X)
                    }
                    glutin::event::VirtualKeyCode::Y => {
                        om.input_manager.release_key(input_manager::Key::Y)
                    }
                    glutin::event::VirtualKeyCode::Z => {
                        om.input_manager.release_key(input_manager::Key::Z)
                    }

                    glutin::event::VirtualKeyCode::Left => {
                        om.input_manager.release_key(input_manager::Key::LEFT)
                    }
                    glutin::event::VirtualKeyCode::Right => {
                        om.input_manager.release_key(input_manager::Key::RIGHT)
                    }
                    glutin::event::VirtualKeyCode::Up => {
                        om.input_manager.release_key(input_manager::Key::UP)
                    }
                    glutin::event::VirtualKeyCode::Down => {
                        om.input_manager.release_key(input_manager::Key::DOWN)
                    }
                    _ => (),
                };
            }
        }
        glutin::event::WindowEvent::MouseInput { state, button,.. }=>{
           match button{
               winit::event::MouseButton::Left => {
                   match state{
                       winit::event::ElementState::Pressed=> om.input_manager.press_key(input_manager::Key::MOUSELEFT),
                       winit::event::ElementState::Released=> om.input_manager.release_key(input_manager::Key::MOUSELEFT),
                   }
               },
               winit::event::MouseButton::Right => {
                   match state{
                       winit::event::ElementState::Pressed=> om.input_manager.press_key(input_manager::Key::MOUSERIGHT),
                       winit::event::ElementState::Released=> om.input_manager.release_key(input_manager::Key::MOUSERIGHT),
                   }
               },
               _=>(),
           }
        },
        glutin::event::WindowEvent::CursorMoved { position, .. } => {
            om.input_manager.mouse.position =
                Vector2::<f64>::new(position.x as f64, position.y as f64);
            //           println!("{} {:?}","cursor moved", position);
        }

        _ => return,
    };

    // println!("{:?}", OM.cam.0);
}

fn process_mouse_input(
    om: &mut object_manager::ObjectManager,
    event: &glutin::event::DeviceEvent,
    deltatime: f64,
) {
    match *event {
        
        glutin::event::DeviceEvent::MouseMotion { delta, .. } => {
            om.input_manager.mouse.hasmoved = true;
            om.input_manager.mouse.delta = Vector2::new(delta.0, delta.1);
        }
        _ => (),
    }
}
