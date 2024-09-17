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
uniform vec2 canvas_origin;

out vec4 FragColor;

float ease(float x) {
    if (x == 0) {
        return 0;
    } else {
        return pow(2, 10 * x - 10);
    }
}

void main() {
    if (!(viewport.x <= ScreenPos.x && ScreenPos.x <= viewport.x + viewport.z &&
          viewport.y <= ScreenPos.y && ScreenPos.y <= viewport.y + viewport.w)) {
        discard;
    }

    // Aspect ratio correction
    float aspectRatio = screen_size.x / screen_size.y;
    vec2 correctedScreenPos = vec2(ScreenPos.x * aspectRatio, ScreenPos.y);

    // Adjust the screen position by the canvas origin
    vec2 adjustedScreenPos = correctedScreenPos - canvas_origin;

    // Step 1: map the origin and adjustedScreenPos into grid space
    vec2 origin = vec2(
        (offset.x * cos(rotation)) - (offset.y * sin(rotation)),
        (offset.y * cos(rotation)) + (offset.x * sin(rotation))
    );

    vec2 thisPoint = vec2(
        (adjustedScreenPos.x * cos(rotation)) - (adjustedScreenPos.y * sin(rotation)),
        (adjustedScreenPos.y * cos(rotation)) + (adjustedScreenPos.x * sin(rotation))
    );

    vec2 diff = thisPoint - origin;

    // Adjust for spacing with zoom and aspect ratio
    float adjustedSpacingX = (spacing / screen_size.x) * zoom * aspectRatio;
    float adjustedSpacingY = (spacing / screen_size.y) * zoom;

    vec2 cell_local = vec2(
        mod(diff.x, adjustedSpacingX) / adjustedSpacingX,
        mod(diff.y, adjustedSpacingY) / adjustedSpacingY
    );

    vec2 cell_d = abs(cell_local - vec2(0.5, 0.5));

    float final_dist = ease(ease(0.5 + max(
        abs(cell_local.x - 0.5),
        abs(cell_local.y - 0.5)
    )));

    FragColor = vec4(color.rgb, color.a * final_dist);
}



