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
    let window = glutin::WindowBuilder::new();
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
    while !closed {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.05, 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => (),
                }
            }
            _ => (),
        });
    }
}
