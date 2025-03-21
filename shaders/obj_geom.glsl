#version 330 core

layout (points) in;
layout (line_strip, max_vertices = 4) out;

out vec3 color;

void main() {
    vec4 pos = gl_in[0].gl_Position;
    gl_Position = pos;
    //color = vec3(1.0, 0.0, 0.0);
    gl_Position = pos + vec4(-0.1, -0.1, 0.0, 0.0);
    color = vec3(1.0, 0.0, 0.0);
    EmitVertex();
    gl_Position = pos + vec4(-0.1, 0.1, 0.0, 0.0);
    color = vec3(1.0, 1.0, 0.0);
    EmitVertex();
    gl_Position = pos + vec4(0.1, 0.1, 0.0, 0.0);
    color = vec3(0.0, 1.0, 0.0);
    EmitVertex();
    gl_Position = pos + vec4(0.1, -0.1, 0.0, 0.0);
    color = vec3(0.0, 0.0, 1.0);
    EmitVertex();
    /*for (int i = 0; i < 8; i++) {
        float cos = cos(6.28*i/8);
        float sin = sin(6.28*i/8);
        gl_Position = pos + vec4(0.2*cos, 0.2*sin, 0.0, 0.0);
        color = vec3(1.0*cos, 1.0*sin, 1.0);
        EmitVertex();
    }*/
    EndPrimitive();
}
