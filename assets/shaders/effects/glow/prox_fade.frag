#version 330 core

in vec2 ScreenPos;

uniform vec4 color;
uniform vec4 viewport;
uniform vec2 mouse;
uniform vec2 screen_size;

out vec4 FragColor;

float dist(vec2 a, vec2 b) {
    vec2 screenA = (a * 0.5 + 0.5) * screen_size;
    vec2 screenB = (b * 0.5 + 0.5) * screen_size;

    return distance(screenA, screenB) / 1024.0;
}

void main() {
    if (!(viewport.x <= ScreenPos.x && ScreenPos.x <= viewport.x + viewport.z &&
          viewport.y <= ScreenPos.y && ScreenPos.y <= viewport.y + viewport.w)) {
        discard;
    }

    if (color.a < 0.1)
        discard;

    float d = dist(mouse, ScreenPos)*2;
    FragColor = vec4(color.rgb * (1.0 - clamp(d, 0.0, 0.5)), color.a);
}
