#version 330 core

in vec2 TexCoord;
in vec2 ScreenPos;

uniform sampler2D atlas;
uniform vec4 uv;
uniform vec4 viewport;
uniform vec4 draw_clip;
uniform vec4 color;
uniform mat4 camera;
uniform mat4 transform;

out vec4 FragColor;

void main() {

    if (!(viewport.x <= ScreenPos.x && ScreenPos.x <= viewport.x+viewport.z && viewport.y <= ScreenPos.y && ScreenPos.y <= viewport.y+viewport.w)) {
        discard;
    }

    vec4 draw_c = vec4(draw_clip.xy, 0.0, 1.0) * (camera * transform);
    vec4 draw_s = vec4(draw_clip.zw, 0.0, 1.0) * (camera * transform);

//    draw_c = vec4(draw_c.xy, draw_s.xy);
//    if (!(draw_c.x <= ScreenPos.x && ScreenPos.x <= draw_c.x+draw_c.z && draw_c.y <= ScreenPos.y && ScreenPos.y <= draw_c.y+draw_c.w)) {
//        FragColor = vec4(1, 0, 0, 1);
//        return;
//    }

    vec2 atlasTexCoord = vec2(
        uv.x + TexCoord.x * uv.z,
        uv.y + TexCoord.y * uv.w
    );

    vec4 col = texture(atlas, atlasTexCoord);
    if (col.a < 0.01) {
        discard;
    }

    col = vec4(color.rgb, col.a * color.a);
    if (col.a < 0.01) {
        discard;
    }

    FragColor = col;

}