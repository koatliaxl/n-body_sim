#version 330
in vec2 TexCoords;
out vec4 frag_color;

uniform sampler2D glyph;
uniform vec3 text_color;

void main() {
    //vec4 sampled = vec4(1.0, 1.0, 1.0, texture(glyph, vec2(TexCoords.x, 1.0 - TexCoords.y)).r);
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture(glyph, TexCoords).r);
    frag_color = vec4(text_color, 1.0) * sampled;
}
