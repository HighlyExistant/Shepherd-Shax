#version 460
layout(location = 0) in vec4 color_i;
layout(location = 0) out vec4 f_color;
layout( push_constant ) uniform constants
{
    mat4 transform;
    mat4 normal;
} pc;
layout(set = 0, binding = 0) uniform UBO {
    vec2 resolution;
} ubo;

void main() {
    f_color = color_i;
    // f_color = color_i;
}