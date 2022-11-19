#version 400

varying lowp vec2 texcoord;
varying lowp float fragment_light_level;
flat in float fragment_texture_index;

uniform sampler2D tex;

void main() {
    ivec2 atlas_size = textureSize(tex, 0);
    gl_FragColor = texture(
        tex,
        vec2(
            texcoord.x,
            (texcoord.y + fragment_texture_index) * atlas_size.x / atlas_size.y
        )
    ) * fragment_light_level;
}
