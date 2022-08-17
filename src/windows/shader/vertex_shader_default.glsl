#version 430

uniform uvec2 resolution;

layout (location = 0) in vec3 vPos;
layout (location = 1) in vec3 vColor;
layout (location = 2) in vec2 vTexCoord;
uniform vec3 center;
uniform float angle;

out vec3 outColor;

float map(float x, float in_min, float in_max, float out_min, float out_max) {
    return (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
}

vec2 rotate(vec2 pos, float angle) {
    float s = sin(angle);
    float c = cos(angle);
    mat2 rot = mat2(c, -s, s, c);
    return rot * pos;
}

void main() {
    vec2 pos = vPos.xy;
    pos = rotate(pos, angle);

    gl_Position = vec4(
        map(center.x + pos.x, 0.0, resolution.x, -1.0, 1.0), 
        map(center.y + pos.y, 0.0, resolution.y, -1.0, 1.0),
        center.z + vPos.z, 
        1.0
    );

    outColor = vColor;
}