#version 150 core

in vec2 pos;
in vec2 tex_coords;

out vec2 v_tex_coords;

void main() {
    v_tex_coords = tex_coords;
    gl_Position = vec4(pos, 0.0, 1.0);
}
