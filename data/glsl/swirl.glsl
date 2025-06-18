uniform shader image;
uniform float2 center;
uniform float strength;
uniform float radius;

half4 main(float2 coord) {
    float2 offset = coord - center;
    float distance = length(offset);
    float angle = strength * (radius - distance) / radius;
    float s = sin(angle);
    float c = cos(angle);
    float2 rotated = float2(c * offset.x - s * offset.y, s * offset.x + c * offset.y);
    return image.eval(center + rotated);
}
