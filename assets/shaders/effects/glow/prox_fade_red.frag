#version 330 core

in vec2 TexCoord;
in vec2 ScreenPos;

uniform sampler2D atlas;
uniform vec4 uv;
uniform vec4 viewport;
uniform vec2 mouse;

out vec4 FragColor;

void main() {

    if (!(viewport.x <= ScreenPos.x && ScreenPos.x <= viewport.x+viewport.z && viewport.y <= ScreenPos.y && ScreenPos.y <= viewport.y+viewport.w)) {
        discard;
    }
    
    vec2 atlasTexCoord = vec2(
        uv.x + TexCoord.x * uv.z,
        uv.y + TexCoord.y * uv.w
    );

    vec4 col = texture(atlas, atlasTexCoord);

    if (col.a < 0.01) {
        discard;
    }
    float dist = distance(mouse, ScreenPos);
    if (col.g >= 0.5) {
        FragColor = vec4(col.rgb * (1-clamp(dist*2, 0, 0.5)), col.a);
    } else {
        FragColor = vec4(col.rgb, col.a * (1-clamp(dist*2, 0, 1)));
    }
}