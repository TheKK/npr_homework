#version 150 core

in vec2 v_tex_coords;

out vec4 o_color;

uniform sampler2D stroke_outline_tex;
uniform sampler2D stroke_ink_quantity_tex;

#define BRUSH_NUM 5

void main() {
    vec4 base_color = texture(stroke_outline_tex, v_tex_coords);
    float ink_quantity = texture(stroke_ink_quantity_tex, v_tex_coords).r;

    // No ink remains.
    if (ink_quantity == 0.0) {
        discard;
    }

    // TODO Apply ink_quantity into computation.
    o_color = base_color;
}
