#version 330 core

in vec2 TexCoord;  // Incoming texture coordinates from the vertex shader
out vec4 FragColor;

uniform sampler2D atlas;  // The texture atlas
uniform vec4 uv; // (startX, startY, width, height)

void main() {
    // Adjust the texture coordinates to fit within the specified sub-region
    vec2 atlasTexCoord = vec2(
        uv.x + TexCoord.x * uv.z, // X start + scaled width
        uv.y + TexCoord.y * uv.w  // Y start + scaled height
    );

    // Sample the atlas using the adjusted texture coordinates
    vec4 col = texture(atlas, atlasTexCoord);

    if (col.a < 0.01) {
        discard;
    }
    FragColor = col;

}