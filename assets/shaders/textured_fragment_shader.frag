#version 330 core

in vec2 TexCoord;
out vec4 FragColor;

uniform sampler2D atlas;
uniform vec4 uv;

void main() {
    vec2 atlasTexCoord = vec2(
        uv.x + TexCoord.x * uv.z,
        uv.y + TexCoord.y * uv.w
    );

    vec4 col = texture(atlas, atlasTexCoord);

    if (col.a < 0.01) {
        discard;
    }
    FragColor = col;

}