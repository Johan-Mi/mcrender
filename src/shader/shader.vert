#version 100

attribute vec4 pos;

uniform mat4 view;

varying lowp vec3 worldcoord;

void main() {
    gl_Position = view * pos;
    worldcoord = pos.xyz;
}