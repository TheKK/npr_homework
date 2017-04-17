#version 150 core

in vec2 v_tex_coords;

out vec4 o_color;

uniform sampler2D previous_tex;

uniform float radius;
uniform vec2 center;
uniform vec4 brush_color;

// TODO Don't copy paste please...
vec4 blend(vec3 base, vec3 blend, float opacity) {
    return vec4(((base * blend) * opacity) + (base * (1.0 - opacity)), 1.0);
}

void main() {
    vec4 previous_color = texture(previous_tex, v_tex_coords);

    vec2 pos = gl_FragCoord.xy;
    float dist = distance(pos, center);

    // if no previous color, set it as white.
    if (length(previous_color) == 0) {
        previous_color = vec4(1.0);
    }

    if (dist <= radius) {
        o_color = blend(previous_color.rgb, brush_color.rgb, brush_color.a);
    } else {
        discard;
    }
}
