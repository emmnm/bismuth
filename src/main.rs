#[macro_use]
extern crate glium;
extern crate rand;

mod geometry;
mod shaders;
mod interpreters;
mod lib;

use lib::LSystem;
use std::collections::HashMap;

use shaders::{VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC};

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_title("Bismuth");
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut lsys: LSystem = LSystem::new("X".to_string());
    lsys.add('F');
    lsys.add_constant('+');
    lsys.add_constant('-');
    lsys.add_constant('[');
    lsys.add_constant(']');
    let mut rule: HashMap<char, String> = HashMap::new();
    rule.insert('X', "F[-X][X]F[-X]+FX".to_string());
    rule.insert('F', "FF".to_string());
    lsys.push(rule);

    // Iterate 6 times
    for _ in 0..1000 {
        lsys = lsys.next_rand();
    }
    println!("lsys: {}", lsys.to_string());

    let shape = interpreters::from_fractal_plant(lsys.to_string());

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

    let program =
        glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None)
            .unwrap();

    let mut closed = false;
    let mut displacement_x: f32 = 0.0;
    let mut displacement_y: f32 = 0.0;
    while !closed {
        let uniforms =
            uniform! {
            view_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [displacement_x, displacement_y, 0.0, 1.0]
            ]
        };
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.05, 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::Closed => closed = true,
                    glutin::WindowEvent::KeyboardInput {
                        device_id: _,
                        input,
                    } => {
                        println!("Key: {:?}", input.virtual_keycode.unwrap());
                        let code = input.virtual_keycode.unwrap();
                        match code {
                            glutin::VirtualKeyCode::Up => displacement_y -= 0.05,
                            glutin::VirtualKeyCode::Down => displacement_y += 0.05,
                            glutin::VirtualKeyCode::Left => displacement_x += 0.05,
                            glutin::VirtualKeyCode::Right => displacement_x -= 0.05,
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        });
    }
}
