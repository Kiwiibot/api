uniform shader image;
uniform float2 center;
uniform float strength;
uniform float radius;

half4 main(float2 coord) {
    float2 offset = coord - center;
    float distance = length(offset);
    offset *= mix(1.0, smoothstep(0.0, radius / distance, distance / radius), strength);
    return image.eval(center + offset);
}
