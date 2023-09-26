#![windows_subsystem = "windows"]

use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

use vector::Vector;

fn main() {
    let window = Window::<()>::new_centered("Pendulum", (300, 180)).unwrap();

    window.run_loop(MyWindowHandler{
        p: Pendulum::new(150.0, 0.0, 100.0)
    });
}

struct MyWindowHandler {
    p: Pendulum
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {

        graphics.clear_screen(Color::BLACK);
        
        self.p.update();
        self.p.draw(graphics);
        
        helper.request_redraw();
    }
}

struct Pendulum {

    origin: Vector,
    position: Vector,

    angle: f32,

    angular_velocity: f32,
    angular_aacceleration: f32,

    r: f32,
    g: f32
}

impl Pendulum {

    fn new(x: f32, y: f32, r: f32) -> Pendulum {
        Pendulum {
            origin: Vector::new(x, y),
            position: Vector::new(x, y),
            angle: 45.0,
            angular_velocity: 0.0,
            angular_aacceleration: 0.0,

            r,
            g: 0.85
        }
        
    }

    fn update(&mut self) {
        self.angular_aacceleration = -1.0 * self.g * self.angle.sin() / self.r;
        self.angular_velocity += self.angular_aacceleration;
        self.angle += self.angular_velocity;

        self.position.set(self.r * self.angle.sin(), self.r * self.angle.cos());
        self.position.add(&self.origin);
    }

    fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_line((self.origin.x, self.origin.y), (self.position.x, self.position.y), 3.0, Color::WHITE,
        );

        graphics.draw_circle((self.position.x, self.position.y), 5.0, Color::WHITE);
    }
}

mod vector {
    pub struct Vector {
        pub x: f32,
        pub y: f32
    }

    impl Vector {
        pub fn new(x: f32, y: f32) -> Vector {
            Vector { x, y }
        }

        pub fn add(&mut self, other: &Vector) -> &Vector {
            self.x += other.x;
            self.y += other.y;
            self
        }

        pub fn set(&mut self, x: f32, y: f32) -> &Vector {
            self.x = x;
            self.y = y;
            self
        }
    }
}