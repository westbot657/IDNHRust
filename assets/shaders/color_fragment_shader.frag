#version 330 core

in vec2 ScreenPos;

uniform vec4 color;
uniform vec4 viewport;
uniform vec2 mouse;
uniform vec2 screen_size;

out vec4 FragColor;

void main() {
    if (!(viewport.x <= ScreenPos.x && ScreenPos.x <= viewport.x+viewport.z &&
          viewport.y <= ScreenPos.y && ScreenPos.y <= viewport.y+viewport.w)) {
        discard;
    }

    if (color.a < 0.1)
        discard;
    FragColor = color;
}