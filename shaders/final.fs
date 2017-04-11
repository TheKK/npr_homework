#version 150 core

in vec2 v_tex_coords;

out vec4 o_color;

uniform sampler2D tex;

void main() {
    vec2 tex_coords = v_tex_coords;
    tex_coords.y *= -1;

    o_color = texture(tex, tex_coords);
}
