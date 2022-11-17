#version 100

varying lowp vec3 worldcoord;

uniform sampler2D tex;

void main() {
    gl_FragColor = texture2D(tex, vec2(1.0, 0.0) - fract(worldcoord.xy));
}
