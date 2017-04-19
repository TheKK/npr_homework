#version 150 core

in vec2 v_tex_coords;

out vec4 o_color;

uniform sampler2D current_tex;
uniform vec2 stroke_start_pos;
uniform vec2 stroke_vector;

void main() {
    // TODO Add actual implementation.
    vec4 color = texture(current_tex, v_tex_coords);
    color.r = 1 - color.r;
    color.g = 1 - color.g;
    color.b = 1 - color.b;

    o_color = color;
}
