#version 330 core
layout (location = 0) in vec3 aPos;

uniform mat4 transform;
uniform mat4 camera;

out vec2 ScreenPos;

void main() {

    vec4 pos = camera * transform * vec4(aPos, 1.0);

    gl_Position = pos;
    ScreenPos = pos.xy;
}