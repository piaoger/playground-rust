extern crate cgmath;
// inspired from glium/examples/support
use glium::glutin;

use self::cgmath::{Point3, Vector3, Matrix4, InnerSpace};

use std::f32::consts::{PI, FRAC_PI_2};

pub struct CameraState {
    aspect_ratio: f32,
    position: Point3<f32>,
    azimuth: f32,
    inclination: f32,
    radius: f32,

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,
    dragging: bool,
    zooming:bool,
    mouse_position: Option<(i32, i32)>,
}

impl CameraState {
    pub fn new() -> CameraState {
        CameraState {
            aspect_ratio: 1024.0 / 768.0,
            position: Point3::new(0.0, 0.0, 0.0),
            radius: 3.,
            // direction: (0.0, 0.0, -1.0),
            azimuth: 0.,
            inclination: 0.,
            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
            dragging: false,
            zooming:false,
            mouse_position: None,
        }
    }

    pub fn reset(&mut self) {
        self.aspect_ratio = 1024.0 / 768.0;
        self.position = Point3::new(0.0, 0.0, 0.0);
        self.radius = 3.;
            // direction: (0.0, 0.0, -1.0);
        self.azimuth = 0.;
        self.inclination = 0.;
        self.moving_up = false;
        self.moving_left = false;
        self.moving_down = false;
        self.moving_right = false;
        self.moving_forward = false;
        self.moving_backward = false;
        self.dragging = false;
        self.zooming = false;
        self.mouse_position = None;
    }

    // pub fn set_position(&mut self, pos: (f32, f32, f32)) {
    //     self.position = pos;
    // }
    //
    // pub fn set_direction(&mut self, dir: (f32, f32, f32)) {
    //     self.direction = dir;
    // }

    pub fn direction(&self) -> Vector3<f32> {
        let t = self.inclination;
        let f = self.azimuth;
        Vector3::new(f.cos() * t.cos(), t.sin(), f.sin() * t.cos())
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        let fov: f32 = PI / 3.0;
        let zfar = 1024.0;
        let znear = 0.1;

        cgmath::perspective(cgmath::Rad::new(fov), self.aspect_ratio, znear, zfar).into()
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        Matrix4::look_at(self.position + (-self.radius * self.direction()),
                         self.position,
                         Vector3::new(0.0, 1.0, 0.0))
            .into()
    }

    pub fn update(&mut self) {
        let f = self.direction();
        let up = Vector3::new(0.0, 1.0, 0.0);

        let s = f.cross(up).normalize();
        let u = s.cross(f);

        if self.moving_up {
            self.position += u * 0.01;
        }

        if self.moving_left {
            self.position += -s * 0.01;
        }

        if self.moving_down {
            self.position += -u * 0.01;
        }

        if self.moving_right {
            self.position += s * 0.01;
        }

        if self.moving_forward {
            self.position += f * 0.01;
        }

        if self.moving_backward {
            self.position += -f * 0.01;
        }
    }

    pub fn process_input(&mut self, event: &glutin::Event) {
        match event {
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed,
                                          _,
                                          Some(glutin::VirtualKeyCode::Up)) => {
                self.moving_up = true;
            }
            &glutin::Event::KeyboardInput(glutin::ElementState::Released,
                                          _,
                                          Some(glutin::VirtualKeyCode::Up)) => {
                self.moving_up = false;
            }
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed,
                                          _,
                                          Some(glutin::VirtualKeyCode::Down)) => {
                self.moving_down = true;
            }
            &glutin::Event::KeyboardInput(glutin::ElementState::Released,
                                          _,
                                          Some(glutin::VirtualKeyCode::Down)) => {
                self.moving_down = false;
            }

            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed,
                                          _,
                                          Some(glutin::VirtualKeyCode::Left)) => {
                self.moving_left = true;
            }
            &glutin::Event::KeyboardInput(glutin::ElementState::Released,
                                          _,
                                          Some(glutin::VirtualKeyCode::Left)) => {
                self.moving_left = false;
            }

            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed,
                                          _,
                                          Some(glutin::VirtualKeyCode::Right)) => {
                self.moving_right = true;
            }
            &glutin::Event::KeyboardInput(glutin::ElementState::Released,
                                          _,
                                          Some(glutin::VirtualKeyCode::Right)) => {
                self.moving_right = false;
            }

            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed,
                                          _,
                                          Some(glutin::VirtualKeyCode::A)) => {
                self.moving_left = true;
            }
            &glutin::Event::KeyboardInput(glutin::ElementState::Released,
                                          _,
                                          Some(glutin::VirtualKeyCode::A)) => {
                self.moving_left = false;
            }
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed,
                                          _,
                                          Some(glutin::VirtualKeyCode::D)) => {
                self.moving_right = true;
            }
            &glutin::Event::KeyboardInput(glutin::ElementState::Released,
                                          _,
                                          Some(glutin::VirtualKeyCode::D)) => {
                self.moving_right = false;
            }
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed,
                                          _,
                                          Some(glutin::VirtualKeyCode::W)) => {
                self.moving_forward = true;
            }
            &glutin::Event::KeyboardInput(glutin::ElementState::Released,
                                          _,
                                          Some(glutin::VirtualKeyCode::W)) => {
                self.moving_forward = false;
            }
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed,
                                          _,
                                          Some(glutin::VirtualKeyCode::S)) => {
                self.moving_backward = true;
            }
            &glutin::Event::KeyboardInput(glutin::ElementState::Released,
                                          _,
                                          Some(glutin::VirtualKeyCode::S)) => {
                self.moving_backward = false;
            }
            &glutin::Event::MouseInput(glutin::ElementState::Pressed,
                                       glutin::MouseButton::Left) => {
                self.dragging = true;
            }
            &glutin::Event::MouseInput(glutin::ElementState::Released,
                                       glutin::MouseButton::Left) => self.dragging = false,
                                      &glutin::Event::MouseMoved(x, y) => {
                let prev = self.mouse_position;
                self.mouse_position = Some((x, y));
                if self.dragging {
                    if let Some((px, py)) = prev {
                        // update direction
                        self.azimuth -= (px - x) as f32 * 0.01;
                        self.inclination += (py - y) as f32 * 0.01;
                        self.inclination =
                            self.inclination.max(-FRAC_PI_2 + 0.01).min(FRAC_PI_2 - 0.01);
                    }
                } else if self.zooming {

                    if let Some((px, py)) = prev {
                        // update direction
                        let offset = (py - y) as f32 * 0.01;
                        let f = self.direction();
                        self.position += -f * offset;
                    }
                }
            }

            &glutin::Event::MouseInput(glutin::ElementState::Pressed,
                                       glutin::MouseButton::Right) => {
                self.zooming = true;
            }
            &glutin::Event::MouseInput(glutin::ElementState::Released,
                                       glutin::MouseButton::Right) => self.zooming = false,


            &glutin::Event::MouseWheel(glutin::MouseScrollDelta::PixelDelta(u, v),_) => {
                let offset = v * 0.01 ;
                let f = self.direction();
                self.position += -f * offset;
            },

            _ => {}
        }
    }
}
