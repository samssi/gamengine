#version 330 core
out vec4 FragColor;

in vec2 v_texture_coordinates;

uniform sampler2D u_texture;

void main() {
    FragColor = texture2D(u_texture, v_texture_coordinates);
}