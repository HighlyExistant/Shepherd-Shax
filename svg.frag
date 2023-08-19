#version 460
layout(location = 0) out vec4 f_color;

struct Curve {
    vec2 p[3];
};
layout(set = 0, binding = 0) uniform UBO {
    vec2 resolution;
} ubo;
layout(binding = 1)  buffer ObjectBuffer {
    int size; // Size 4: Align: 0
	Curve objects[]; // Size 24 Align: 8
} curves;

float dist_to_line(vec2 a, vec2 b, vec2 p)
{
	vec2 dir = b - a;
	vec2 norm = vec2(-dir.y, dir.x);
	return dot(normalize(norm), a - p);
}
float calc_t(vec2 a, vec2 b, vec2 p)
{
	vec2 dir = b - a;
	float t = dot(p - a, dir) / dot(dir, dir);
	return clamp(t, 0.0, 1.0);
}
float dist_to_bezier3(vec2 p0, vec2 p1, vec2 p2, vec2 p3, vec2 p)
{
	float t = calc_t(p0, p3, p);
	vec2 q0 = mix(p0, p1, t);
	vec2 q1 = mix(p1, p2, t);
	vec2 q2 = mix(p2, p3, t);
	vec2 r0 = mix(q0, q1, t);
	vec2 r1 = mix(q1, q2, t);
	return dist_to_line(r0, r1, p);
}
float dist_to_bezier2(vec2 p0, vec2 p1, vec2 p2, vec2 p)
{
	float t = calc_t(p0, p2, p);
	vec2 q0 = mix(p0, p1, t);
	vec2 q1 = mix(p1, p2, t);
	return dist_to_line(q0, q1, p);
}
float dist_to_bezier2(vec2 p0, vec2 p1, vec2 p2, float t, vec2 p)
{
	vec2 q0 = mix(p0, p1, t);
	vec2 q1 = mix(p1, p2, t);
	return dist_to_line(q0, q1, p);
}
#define UDIST_BIAS 0.001
void process_bezier2(vec2 p, int i, inout float min_udist, inout float v)
{
	vec2 p0 = curves.objects[i].p[0];
	vec2 p1 = curves.objects[i].p[1];
	vec2 p2 = curves.objects[i].p[2];

	float t = calc_t(p0, p2, p);
	float udist = distance(mix(p0, p2, t), p);

	if (udist <= min_udist + UDIST_BIAS)
	{
		float bez = dist_to_bezier2(p0, p1, p2, t, p);

		if (udist >= min_udist - UDIST_BIAS)
		{
			vec2 prevp = curves.objects[i].p[2];
			float prevd = dist_to_line(p0, p2, prevp);
			v = mix(min(bez, v), max(bez, v), step(prevd, 0.0));
		}
		else
		{
			v = bez;
		}

		min_udist = min(min_udist, udist);
	}

}

void main() {
    vec2 resolution_coord = gl_FragCoord.xy/ubo.resolution;
    float min_udist = 1.0 / 0.0;
    float v = -1.0 / 0.0;
    for (int i = 0; i < curves.size; i += 2) {
        process_bezier2(resolution_coord, i, min_udist, v);
    }
    f_color = vec4(min_udist);
}