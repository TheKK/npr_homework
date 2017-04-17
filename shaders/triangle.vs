#version 150 core

in vec2 pos;

out vec2 v_tex_coords;

void main() {
    v_tex_coords = vec2((pos.x + 1) / 2, (pos.y + 1) / 2);
    gl_Position = vec4(pos, 0.0, 1.0);
}