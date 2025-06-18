uniform shader image;
uniform float time;
uniform float amplitude;
uniform float frequency;

half4 main(float2 coord) {
    float wave = sin(coord.y * frequency + time) * amplitude;
    return image.eval(coord + float2(wave, 0.0));
}
