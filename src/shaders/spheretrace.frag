#version 430

in vec2 fragTexCoord;

uniform vec3 u_cameraPos;
uniform vec3 u_cameraTarget;
uniform vec3 u_cameraUp;

uniform int u_screenWidth;
uniform int u_screenHeight;

uniform float u_fov;

uniform float u_nearClipDist;

struct Sphere {
    vec3 center;
    float radius;
};

layout(std430, binding = 0) buffer Spheres {
    Sphere b_spheres[];
};
uniform int u_sphereCount;

out vec4 finalColor;

struct Ray {
    vec3 position;
    vec3 direction;
};

void main() {
    // Normalized screen coordinates (0..1)
    vec2 uv = fragTexCoord;

    float aspectRatio = float(u_screenWidth) / float(u_screenHeight);

    vec3 cameraForward = normalize(u_cameraTarget - u_cameraPos);
    vec3 cameraRight = normalize(cross(u_cameraUp, cameraForward));

    float halfHeight = tan(u_fov * 0.5);
    float halfWidth = halfHeight * aspectRatio;

    // Calculate frustum stuff
    vec2 ndc = uv * 2.0 - 1.0;

    vec3 rayOrigin = u_cameraPos;
    vec3 rayDir = normalize(
        cameraForward
        + ndc.x * halfWidth * cameraRight
        + ndc.y * halfHeight * u_cameraUp
    );

    // Calculate hit
    float closestT = 1.0 / 0.0; // Dividing by 0 gives inf, we question not the ways of this world
    vec3 hitNormal = vec3(0.0);
    bool hitSomething = false;

    for (int i = 0; i < u_sphereCount; i++) {
        Sphere s = b_spheres[i];

        vec3 v = rayOrigin - s.center;
        float b = dot(v, rayDir);
        float c = dot(v, v) - s.radius * s.radius;
        float d = b * b - c;

        if ((c > 0.0 && b > 0.0) || d < 0.0) {
            continue;
        }

        float t = max(0.0, -b - sqrt(d));

        if (t >= closestT) {
            continue;
        }

        closestT = t;
        vec3 position = rayOrigin + rayDir * t;
        hitNormal = (position - s.center) / s.radius;
        hitSomething = true;
    }

    if (hitSomething) {
        finalColor = vec4(hitNormal * 0.5 + 0.5, 1.0);
    } else {
        finalColor = vec4(1.0); // background
    }
}
