#version 330

layout(location=0) in vec2 pos;
layout(location=1) in vec2 tex_coords;

out vec2 TexCoords;

uniform mat4 proj_mat;

void main() {
    gl_Position = proj_mat*vec4(pos, 0.0, 0.0);
    TexCoords = tex_coords;
}
