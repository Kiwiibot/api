uniform shader image;
uniform float2 center;
uniform float strength;
uniform float radius;
uniform float zoom;

half4 main(float2 coord) {
    float2 zoomed = center + (coord - center) / zoom;
    float2 offset = zoomed - center;
    float dist = length(offset);
    if (dist < radius) {
        float2 norm = normalize(offset + 0.0001);
        float refraction = strength * (radius - dist) / radius;
        float2 refract_offset = norm * refraction * dist * 0.5;

        half4 color;
        color.r = image.eval(zoomed + refract_offset + float2(2.0, 0.0)).r;
        color.g = image.eval(zoomed + refract_offset).g;
        color.b = image.eval(zoomed + refract_offset - float2(2.0, 0.0)).b;
        color.a = image.eval(zoomed + refract_offset).a;
        return color;
    }
    return image.eval(zoomed);
}
