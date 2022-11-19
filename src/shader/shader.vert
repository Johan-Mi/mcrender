#version 400

attribute vec4 pos;
attribute vec2 uv;
attribute float light_level;

uniform mat4 view;

varying lowp vec2 texcoord;
varying lowp float fragment_light_level;

void main() {
    gl_Position = view * pos;
    texcoord = uv;
    fragment_light_level = light_level;
}
