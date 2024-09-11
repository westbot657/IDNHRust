#version 330 core
out vec4 FragColor;

uniform vec4 color;

void main() {
    if (color.a < 0.1)
        discard;
    FragColor = color;
}