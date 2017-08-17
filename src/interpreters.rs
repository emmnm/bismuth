use geometry::Vertex;

pub fn from_fractal_plant(s: String) -> Vec<Vertex> {
    // Example 7, wikipedia.

    const ANGLE_SIZE: f32 = 0.43; // about 25 degrees in rads.
    const STEP_SIZE: f32 = 0.05;

    #[derive(Copy, Clone)]
    struct State {
        pub x: f32,
        pub y: f32,
        pub theta: f32,
    };

    let mut current = State {
        x: -1.0,
        y: -1.0,
        theta: 45.0,
    };
    let mut stack = vec![];
    let mut verts = vec![];

    for c in s.chars() {
        match c {
            'X' => (), // partial evaluation
            'F' => {
                let future = State {
                    x: current.x + STEP_SIZE * current.theta.cos(),
                    y: current.y + STEP_SIZE * current.theta.sin(),
                    theta: current.theta,
                };
                verts.push(Vertex::new(current.x, current.y));
                verts.push(Vertex::new(future.x, future.y));
                current = future;
            }
            '-' => (current.theta -= ANGLE_SIZE),
            '+' => (current.theta += ANGLE_SIZE),
            '[' => (stack.push(current)),
            ']' => (current = stack.pop().unwrap()),
            _ => {
                println!("c: {}", c);
                assert!(false)
            }
        };
    }

    verts
}
