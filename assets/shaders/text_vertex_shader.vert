#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

uniform mat4 transform;
uniform mat4 camera;
uniform float italic;

out vec2 TexCoord;
out vec2 ScreenPos;
out float antialiasing;

void main() {

    vec4 pos = vec4((camera * transform) * vec4(aPos.x + (italic * aPos.y / 5.0), aPos.y, aPos.z, 1.0));

    antialiasing = mod(aPos.x - (italic * aPos.y / 5.0), 1.0);
    gl_Position = pos;
    ScreenPos = pos.xy;
    TexCoord = aTexCoord;
}