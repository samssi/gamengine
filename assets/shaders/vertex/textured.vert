#version 330 core
in vec4 a_position;
in vec2 a_texture_coordinates;

uniform mat4 u_matrix;
out vec2 v_texture_coordinates;

void main() {
    gl_Position = u_matrix * a_position;

    v_texture_coordinates = a_texture_coordinates;
}