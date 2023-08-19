#version 460

layout( push_constant ) uniform constants
{
    mat4 transform;
    mat4 normal;
} pc;

layout(location = 0) in vec4 rgba;
layout(location = 1) in vec2 pos;

layout(location = 0) out vec4 color_o;
void main() {
    vec3 position = mat3(pc.transform) * vec3(pos, 1.0);

    gl_Position = vec4(vec2(position), 0.0, 1.0);

    color_o = rgba;
}