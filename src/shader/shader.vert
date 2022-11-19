#version 100

attribute vec4 pos;
attribute vec2 uv;

uniform mat4 view;

varying lowp vec2 texcoord;

void main() {
    gl_Position = view * pos * vec4(-1, 1, 1, 1);
    texcoord = uv;
}
