#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

uniform mat4 transform;
uniform mat4 camera;

out vec2 TexCoord;
out vec2 ScreenPos;

void main() {

    vec4 pos = vec4((camera * transform) * vec4(aPos, 1.0));

    gl_Position = pos;
    ScreenPos = pos.xy;
    TexCoord = aTexCoord;
}