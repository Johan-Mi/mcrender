#version 400

attribute vec4 pos;
attribute vec2 uv;
attribute float light_level;
attribute float texture_index;

uniform mat4 view;

varying lowp vec2 texcoord;
varying lowp float fragment_light_level;
flat out float fragment_texture_index;

void main() {
    gl_Position = view * pos;
    texcoord = uv;
    fragment_light_level = light_level;
    fragment_texture_index = texture_index;
}
