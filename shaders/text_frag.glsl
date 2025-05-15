#version 330
in vec2 TexCoords;
out vec4 color;

uniform sampler2D glyph;
uniform vec3 text_color;

void main() {
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture(glyph, TexCoords).r);
    color = vec4(text_color, 1.0) * sampled;
}
