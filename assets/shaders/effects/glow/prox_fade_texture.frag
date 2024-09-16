#version 330 core

in vec2 TexCoord;
in vec2 ScreenPos;

uniform sampler2D atlas;
uniform vec4 uv;
uniform vec4 viewport;
uniform vec2 mouse;
uniform vec2 screen_size;

out vec4 FragColor;

float dist(vec2 a, vec2 b) {
    vec2 screenA = (a * 0.5 + 0.5) * screen_size;
    vec2 screenB = (b * 0.5 + 0.5) * screen_size;

    return distance(screenA, screenB) / (screen_size.x);
}


void main() {

    if (!(viewport.x <= ScreenPos.x && ScreenPos.x <= viewport.x+viewport.z &&
          viewport.y <= ScreenPos.y && ScreenPos.y <= viewport.y+viewport.w)) {
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
    float dis = dist(mouse, ScreenPos);
    FragColor = vec4(col.rgb * (1-clamp(dis*4, 0, 0.5)), col.a);
}