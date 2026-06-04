#version 140

out vec4 color;

uniform float x;
uniform float y;
uniform float t;

void main(){
    color = vec4(abs(sin(t + x)), abs(sin(t + y)), abs(sin(t + (x + y))),1.0);
}