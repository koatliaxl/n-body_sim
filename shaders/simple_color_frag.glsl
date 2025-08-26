#version 330 core

out vec4 frag_color;
uniform vec3 color;

void main() {
    frag_color = vec4(color, 1.0);
    //frag_color = vec4(1.0, 0.1, 0.2, 1.0);
}
