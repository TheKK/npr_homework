#version 150 core

out vec4 o_color;

uniform float radius;
uniform vec2 center;
uniform vec4 brush_color;

void main() {
    vec2 pos = gl_FragCoord.xy;

    float dist = distance(pos, center);

    if (dist <= radius) {
        o_color = brush_color;
    } else {
        discard;
    }
}
