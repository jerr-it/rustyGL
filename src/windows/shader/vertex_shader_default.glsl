#version 430

uniform uvec2 resolution;

layout (location = 0) in vec3 vPos;
layout (location = 1) in vec3 vColor;
layout (location = 2) in vec2 vTexCoord;
uniform vec3 center;

out vec3 outColor;

float map(float x, float in_min, float in_max, float out_min, float out_max) {
    return (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
}

void main() {
    gl_Position = vec4(
        map(center.x + vPos.x, 0.0, resolution.x, -1.0, 1.0), 
        map(center.y + vPos.y, 0.0, resolution.y, -1.0, 1.0),
        center.z + vPos.z, 
        1.0
    );

    outColor = vColor;
}