#version 100

varying lowp vec3 worldcoord;

uniform sampler2D tex;

void main() {
    lowp vec3 fractionals = fract(worldcoord.xyz);
    lowp vec2 uv = fractionals.x == 0.0 ? fractionals.zy
                 : fractionals.y == 0.0 ? fractionals.xz
                 : fractionals.xy;
    gl_FragColor = texture2D(tex, vec2(1.0, 0.0) - uv);
}
