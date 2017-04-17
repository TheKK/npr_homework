#version 150 core

out vec4 o_color;

uniform vec4 brush_color;

void main() {
    o_color = brush_color;
}
