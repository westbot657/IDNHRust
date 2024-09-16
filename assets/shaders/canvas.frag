#version 330 core

in vec2 ScreenPos;

uniform vec4 color;
uniform vec4 viewport;
uniform vec2 mouse;
uniform vec2 screen_size;

uniform float rotation; // in radians
uniform vec2 offset;
uniform float zoom;
uniform float spacing;

out vec4 FragColor;

float dist(vec2 a, vec2 b) {
    vec2 screenA = (a * 0.5 + 0.5) * screen_size;
    vec2 screenB = (b * 0.5 + 0.5) * screen_size;

    return distance(screenA, screenB) / 8;
}

void main() {
    if (!(viewport.x <= ScreenPos.x && ScreenPos.x <= viewport.x+viewport.z &&
          viewport.y <= ScreenPos.y && ScreenPos.y <= viewport.y+viewport.w)) {
        discard;
    }

    // Step 1: map the origin and screenPos into grid space
    // float d = dist(vec2(0, 0), offset);

    // vec2 origin = vec2(
    //     cos(rotation)*d,
    //     sin(rotation)*d
    // );
    vec2 origin = vec2(
        (offset.x * cos(rotation)) - (offset.y * sin(rotation)),
        (offset.y * cos(rotation)) + (offset.x * sin(rotation))
    );

    // float d2 = dist(vec2(0, 0), ScreenPos);

    // vec2 thisPoint = vec2(
    //     cos(rotation)*d2,
    //     sin(rotation)*d2
    // );
    vec2 thisPoint = vec2(
        (ScreenPos.x * cos(rotation)) - (ScreenPos.y * sin(rotation)),
        (ScreenPos.y * cos(rotation)) + (ScreenPos.x * sin(rotation))
    );

    vec2 diff = thisPoint - origin;

    // vec2 cell_local = (diff % spacing) / spacing;
    vec2 cell_local = vec2(
        mod(diff.x, spacing) / spacing,
        mod(diff.y, spacing) / spacing
    );

    vec2 cell_d = abs(cell_local - vec2(0.5, 0.5));

    float final_dist = max(
        abs(cell_local.x - 0.5),
        abs(cell_local.y - 0.5)
    );

    FragColor = vec4(color.rgb, color.a * final_dist);


}

