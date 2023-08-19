#version 460

layout( push_constant ) uniform constants
{
    mat4 transform;
    mat4 normal;
} pc;

layout(location = 0) in vec2 pos;
layout(location = 1) in vec2 uv;

layout(location = 0) out vec2 uv_o;
void main() {
    vec3 position = mat3(pc.transform) * vec3(pos, 1.0);

    gl_Position = vec4(vec2(position), 0.0, 1.0);

    uv_o = uv;
}