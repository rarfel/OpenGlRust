#version 140

in vec2 position;

uniform float x;
uniform float y;

void main() {
    vec2 pos = position;
    pos.x += x;
    pos.y += y;
    gl_Position = vec4(pos, 0.0, 1.0);
}