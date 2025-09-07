#version 430

in vec2 fragTexCoord;
uniform vec3 u_cameraPos;
uniform vec3 u_cameraTarget;
uniform vec3 u_cameraForward;
uniform vec3 u_cameraUp;
uniform vec3 u_cameraRight;

uniform int u_screenWidth;
uniform int u_screenHeight;

uniform float u_fov;

uniform float u_nearClip;

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
    vec2 uv = gl_FragCoord.xy / vec2(u_screenWidth, u_screenHeight);

    float aspectRatio = float(u_screenWidth) / float(u_screenHeight);

    // Calculate positions
    float halfHeight = tan(u_fov * 0.5) * u_nearClip;
    float halfWidth = halfHeight * aspectRatio;

    vec3 screenCenter = u_cameraPos + u_cameraForward * u_nearClip;

    vec3 pixelPos = screenCenter
        + u_cameraUp * halfHeight * (2.0 * uv.y - 1)
        + u_cameraRight * halfWidth * (2.0 * uv.x - 1);

    vec3 rayDir = normalize(pixelPos - u_cameraPos);

    // Calculate hit
    float closestT = 1e20;
    vec3 hitNormal = vec3(0.0);
    bool hitSomething = false;

    for (int i = 0; i < u_sphereCount; i++) {
        Sphere s = b_spheres[i];

        vec3 v = pixelPos - s.center;
        float b = dot(v, rayDir);
        float c = dot(v, v) - s.radius * s.radius;
        float d = b * b - c;

        if (!((c > 0.0 && b > 0.0) || d < 0.0)) {
            float t = max(0.0, -b - sqrt(d));
            if (t < closestT) {
                closestT = t;
                vec3 position = pixelPos + rayDir * t;
                hitNormal = (position - s.center) / s.radius;
                hitSomething = true;
            }
        }
    }

    if (hitSomething) {
        finalColor = vec4(hitNormal * 0.5 + 0.5, 1.0);
    } else {
        finalColor = vec4(1.0); // background
    }
}
