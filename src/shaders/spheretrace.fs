#version 330

uniform vec3 u_cameraPos;
uniform vec3 u_cameraTarget;
uniform vec3 u_cameraUp;

struct Sphere {
    vec3 center;
    float radius;
};

uniform Sphere u_sphere;

out vec4 finalColor;

struct Ray {
    vec3 position;
    vec3 direction;
};

bool sphere_hit(
    vec3 ray_pos,
    vec3 ray_dir,
    vec3 circle_pos,
    float circle_radius
) {
    vec3 v = ray_pos - circle_pos;
    float b = dot(v, ray_dir);
    float c = dot(v, v) - circle_radius * circle_radius;

    if (c > 0.0 && b > 0.0) {
        return false;
    }

    float d = b * b - c;

    if (d < 0.0) {
        return false;
    }

    float t = -b - sqrt(d);

    if (t < 0.0) {
        t = 0.0;
    }

    return true;

    // let position = ray.position + ray.direction * t;
    // let normal = (position - self.center) / self.radius;
    //
    // Some(RayHit { position, normal, t })
}

void main() {
    // Normalized screen coordinates (0..1)
    vec2 uv = gl_FragCoord.xy / vec2(800, 600);

    // Map to screen plane (-1..1)
    float halfHeight = tan(u_fov * 0.5) * u_nearClip;
    float halfWidth = halfHeight * u_aspectRatio;

    vec3 screenCenter = u_cameraPos + u_cameraForward * u_nearClip;
    vec3 topLeft = screenCenter
        + u_cameraUp * halfHeight
        - u_cameraRight * halfWidth;

    vec3 pixelPos = topLeft
        + u_cameraRight * (uv.x * 2.0 * halfWidth)
        - u_cameraUp * (uv.y * 2.0 * halfHeight);

    vec3 rayDir = normalize(pixelPos - u_cameraPos);

    // Sphere at origin with radius 1
    vec3 sphereCenter = vec3(0.0, 0.0, 0.0);
    float radius = 1.0;

    vec3 oc = u_cameraPos - sphereCenter;
    float a = dot(rayDir, rayDir);
    float b = 2.0 * dot(oc, rayDir);
    float c = dot(oc, oc) - radius*radius;
    float discriminant = b*b - 4.0*a*c;

    vec3 color;
    if (discriminant > 0.0) {
        float t = (-b - sqrt(discriminant)) / (2.0*a);
        vec3 hitPoint = u_cameraPos + rayDir * t;
        vec3 normal = normalize(hitPoint - sphereCenter);
        color = normal * 0.5 + 0.5; // map [-1,1] -> [0,1]
    } else {
        color = vec3(1.0);
    }

    finalColor = vec4(color, 1.0);
}
