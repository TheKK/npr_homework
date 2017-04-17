#version 150 core

#define PI 3.1415926535897932384626433832795

in vec2 v_tex_coords;

out vec4 o_color;

uniform sampler2D stroke_outline_tex;
uniform sampler2D stroke_ink_quantity_tmp_tex;

uniform vec2 start_pos;
uniform vec2 end_pos;

uniform float start_radius;
uniform float end_radius;

uniform float start_ink_quantity;
uniform float end_ink_quantity;

float cross2d(vec2 a, vec2 b) {
    return (a.x * b.y) - (a.y * b.x);
}

mat2 rotate2d(float angle){
    return mat2(cos(angle), -sin(angle),
                sin(angle), cos(angle));
}

bool is_in_circle(vec2 pos, vec2 center, float radius) {
    return distance(pos, center) <= radius;
}

float caculate_area_b_ink_quantity(vec2 pos,
                                   vec2 start_pos, float start_ink_quantity,
                                   vec2 end_pos, float end_ink_quantity) {
    vec2 start_to_end_v_norm = normalize(end_pos - start_pos);
    vec2 start_to_pos_v = pos - start_pos;

    vec2 that_pos = start_pos + start_to_end_v_norm * dot(start_to_pos_v, start_to_end_v_norm);

    float all_length = distance(start_pos, end_pos);
    float that_length = distance(start_pos, that_pos);

    return start_ink_quantity +
        (that_length / all_length) * (end_ink_quantity - start_ink_quantity);
}

bool is_in_area_b(vec2 pos,
                  vec2 start_pos, float start_radius,
                  vec2 end_pos, float end_radius) {

    vec2 start_to_end_v_norm = normalize(end_pos - start_pos);
    vec2 start_to_end_n_norm = rotate2d(PI / 2.0) * start_to_end_v_norm;

    vec2 start_upper_pos = start_pos + start_to_end_n_norm * start_radius;
    vec2 end_upper_pos = end_pos + start_to_end_n_norm * end_radius;

    vec2 start_lower_pos = start_pos - start_to_end_n_norm * start_radius;
    vec2 end_lower_pos = end_pos - start_to_end_n_norm * end_radius;

    vec2 upper_line_v = end_upper_pos - start_upper_pos;
    vec2 lower_line_v = end_lower_pos - start_lower_pos;

    vec2 start_upper_to_pos_v = pos - start_upper_pos;
    vec2 start_lower_to_pos_v = pos - start_lower_pos;

    vec2 start_to_pos_v = pos - start_pos;
    vec2 end_to_pos_v = pos - end_pos;

    return
        (cross2d(upper_line_v, start_upper_to_pos_v) > 0.0) &&
        (cross2d(lower_line_v, start_lower_to_pos_v) < 0.0) &&

        (cross2d(start_to_end_n_norm, start_to_pos_v) > 0.0) &&
        (cross2d(start_to_end_n_norm, end_to_pos_v) < 0.0);
}

void main() {
    bool not_in_stroke_outline = texture(stroke_outline_tex, v_tex_coords).a > 0;
    bool already_painted = texture(stroke_ink_quantity_tmp_tex, v_tex_coords).a > 0;
    if (not_in_stroke_outline || already_painted) {
        discard;
    }

    vec2 pos = gl_FragCoord.xy;

    vec2 start_to_end_v_norm = normalize(end_pos - start_pos);
    vec2 start_to_end_n_norm = rotate2d(PI / 2.0) * start_to_end_v_norm;

    vec2 start_to_pos_v = pos - start_pos;
    vec2 end_to_pos_v = pos - end_pos;

    bool in_area_a = is_in_circle(pos, start_pos, start_radius) &&
                     (cross2d(start_to_end_n_norm, start_to_pos_v) < 0.0);

    bool in_area_c = is_in_circle(pos, end_pos, end_radius) &&
                     (cross2d(start_to_end_n_norm, end_to_pos_v) > 0.0);

    bool in_area_b = !(in_area_a || in_area_c) &&
                     is_in_area_b(pos, start_pos, start_radius, end_pos, end_radius);

    float prev_ink_quantity = texture(stroke_ink_quantity_tmp_tex, v_tex_coords).x;

    float new_ink_quantity = prev_ink_quantity;
    if (in_area_a) {
        new_ink_quantity = (start_ink_quantity + prev_ink_quantity) / 1.0;

    } else if (in_area_c) {
        new_ink_quantity = (end_ink_quantity + prev_ink_quantity) / 1.0;

    } else if (in_area_b) {
        new_ink_quantity = (prev_ink_quantity +
            caculate_area_b_ink_quantity(pos,
                                         start_pos, start_ink_quantity,
                                         end_pos, end_ink_quantity)) / 1.0;
    } else {
        discard;
    }

    o_color = vec4(new_ink_quantity, 0, 1, 1);
}
