#version 400

varying lowp vec2 texcoord;
varying lowp float fragment_light_level;

uniform sampler2D tex;

void main() {
    gl_FragColor = texture(tex, texcoord) * fragment_light_level;
}
