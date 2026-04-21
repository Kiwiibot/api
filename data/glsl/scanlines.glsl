uniform shader image;
uniform float time;

half4 main(float2 coord) {
    half4 colour = image.eval(coord);
    if (mod(coord.y - time * 100.0, 4.0) < 2.0) {
        colour.rgb *= 0.5;
    }
    float shift = sin(time * 6.28318530718 + coord.y * 0.09) * 1.5;
    half r = image.eval(coord + float2(shift, 0.0)).r;
    half b = image.eval(coord - float2(shift, 0.0)).b;
    colour.r = r;
    colour.b = b;
    float2 norm = (coord / 500.0) - 1.0;
    float vignette = 1.0 - dot(norm, norm) * 0.55;
    colour.rgb *= clamp(vignette, 0.3, 1.0);
    return colour;
}