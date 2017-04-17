#version 150 core

in vec2 v_tex_coords;

out vec4 o_color;

uniform sampler2D stroke_ink_quantity_tex;

uniform sampler2D level_0_brush_tex;
uniform sampler2D level_1_brush_tex;
uniform sampler2D level_2_brush_tex;
uniform sampler2D level_3_brush_tex;
uniform sampler2D level_4_brush_tex;

#define BRUSH_NUM 5

void main() {
    float ink_quantity = texture(stroke_ink_quantity_tex, v_tex_coords).r;
    int tex_num = int(ceil(ink_quantity * BRUSH_NUM));
    if (ink_quantity == 0.0) {
        discard;
    }

    switch (tex_num) {
    case 1:
        o_color = 2 * ink_quantity * texture(level_0_brush_tex, v_tex_coords * 900.0 / 32.0);
        break;
    case 2:
        o_color = 2 * ink_quantity * texture(level_1_brush_tex, v_tex_coords * 900.0 / 32.0);
        break;
    case 3:
        o_color = 2 * ink_quantity * texture(level_2_brush_tex, v_tex_coords * 900.0 / 32.0);
        break;
    case 4:
        o_color = 2 * ink_quantity * texture(level_3_brush_tex, v_tex_coords * 900.0 / 32.0);
        break;
    case 5:
        o_color = 2 * ink_quantity * texture(level_4_brush_tex, v_tex_coords * 900.0 / 32.0);
        break;
    default:
        discard;
    }
}
