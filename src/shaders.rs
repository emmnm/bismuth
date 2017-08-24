pub const VERTEX_SHADER_SRC: &str = r#"
#version 140
uniform mat4 view_matrix;
in vec2 position;

void main() {
    gl_Position = view_matrix * vec4(position, 0.0, 1.0);
}
"#;

pub const FRAGMENT_SHADER_SRC: &str = r#"
#version 140
uniform mat4 view_matrix;
out vec4 color;

void main() {
    // have color change with position!
    color = view_matrix * vec4(1.0, 0.0, 0.0, 1.0);
}
"#;
