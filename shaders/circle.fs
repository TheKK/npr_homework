#version 150 core

in vec2 v_center;
in float v_radius;

out vec4 o_color;

uniform float radius;
uniform vec2 center;

void main() {
    vec2 pos = gl_FragCoord.xy;
    float dist = distance(pos, center);

    if (dist <= radius) {
        o_color =  vec4(1, 0, 0, 1);
    } else {
        discard;
    }
}
