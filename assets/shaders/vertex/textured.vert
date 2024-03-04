#version 330 core
out vec2 v_texture_coordinates;

in vec4 a_position;
in vec2 a_texture_coordinates;

uniform mat4 u_matrix;


void main() {
    gl_Position = u_matrix * a_position;

    v_texture_coordinates = a_texture_coordinates;
}