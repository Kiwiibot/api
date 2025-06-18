uniform shader image;
uniform float2 offset;

half4 main(float2 coord) {
    return image.eval(coord + offset);
}
