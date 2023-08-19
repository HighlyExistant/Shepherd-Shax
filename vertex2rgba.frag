#version 460
layout(location = 0) in vec4 color_i;
layout(location = 0) out vec4 f_color;
layout( push_constant ) uniform constants
{
    mat4 transform;
    mat4 normal;
} pc;

void main() {
    f_color = color_i;
}