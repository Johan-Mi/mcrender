#version 100

varying lowp vec2 texcoord;
varying lowp float fragment_light_level;

uniform sampler2D tex;

void main() {
    gl_FragColor = texture2D(tex, texcoord) * fragment_light_level;
}
