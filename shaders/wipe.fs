#version 150 core

#define PI 3.1415926535897932384626433832795

in vec2 v_tex_coords;

out vec4 o_color;

uniform sampler2D current_tex;

uniform vec2 stroke_start_pos;
uniform vec2 stroke_end_pos;

uniform float start_radius;
uniform float end_radius;

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
        (
cross2d(upper_line_v, start_upper_to_pos_v) > 0.0) &&
        (cross2d(lower_line_v, start_lower_to_pos_v) < 0.0) &&

        (cross2d(start_to_end_n_norm, start_to_pos_v) > 0.0) &&
        (cross2d(start_to_end_n_norm, end_to_pos_v) < 0.0);
}

vec4
blend(vec4 base, vec4 blend) {
    return vec4(base.rgb * base.a + blend.rgb * blend.a, 1.0);
}

bool
is_in_stroke(vec2 pos) {
    vec2 stroke_vector = stroke_end_pos - stroke_start_pos;
    vec2 start_to_end_v_norm = normalize(stroke_vector);
    vec2 start_to_end_n_norm = rotate2d(PI / 2.0) * start_to_end_v_norm;

    vec2 start_to_pos_v = pos - stroke_start_pos;
    vec2 end_to_pos_v = pos - stroke_end_pos;

    bool in_area_a = is_in_circle(pos, stroke_start_pos, start_radius) &&
        (cross2d(start_to_end_n_norm, start_to_pos_v) < 0.0);

    bool in_area_c = is_in_circle(pos, stroke_end_pos, end_radius) &&
        (cross2d(start_to_end_n_norm, end_to_pos_v) > 0.0);

    bool in_area_b = (!in_area_a && !in_area_c) &&
        is_in_area_b(pos, stroke_start_pos, start_radius, stroke_end_pos, end_radius);

    return in_area_a || in_area_b || in_area_c;
}

void
main() {
    vec2 pos = gl_FragCoord.xy;
    vec4 old_pigment = texture(current_tex, v_tex_coords);

    if (!is_in_stroke(pos)) {
        // if (old_pigment.a == 0) {
            // discard;
        // } else {
            o_color = old_pigment;
            return;
        // }
    }

    vec2 stroke_vector = stroke_end_pos - stroke_start_pos;
    vec2 start_to_end_v_norm = normalize(stroke_vector);

    float alpha = 1;
    float beta = 120;
    vec2 wipe_vector = start_to_end_v_norm * min(beta, length(stroke_vector)) * alpha;
    float wipe_dist = length(wipe_vector);

    vec2 a, b;
    if (-wipe_vector.x >= 0) {
        a = pos;
        b = pos - wipe_vector;
    } else {
        a = pos - wipe_vector;
        b = pos;
    }

    int y_step;
    if (b.y > a.y) {
        y_step = 1;
    } else {
        y_step = -1;
    }

    float dx = b.x - a.x;
    float dy = b.y - a.y;
    float derror = abs(dy / dx);

    float error = derror - 0.5;

    vec4 sum = vec4(0.0);

    int y = int(round(pos.y));
    for (int x = int(round(a.x)); x < int(round(b.x)); ++x) {
        vec2 wipe_pos = vec2(x, y);
        vec2 wipe_uv = wipe_pos / 900;

        float dist = distance(pos, wipe_pos);

        vec4 wiped_pigment = texture(current_tex, wipe_uv);
        wiped_pigment /= (dist);

        sum += wiped_pigment;

        error += derror;
        if (error >= 0.5) {
            y += y_step;
            error -= 1;
        }
    }

    o_color = old_pigment * vec4(sum.rgb, 1.0);
}
