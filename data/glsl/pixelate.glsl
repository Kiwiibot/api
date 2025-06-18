uniform shader image;
uniform float2 pixel_size;

half4 main(float2 coord) {
    float2 pixel = floor(coord / pixel_size) * pixel_size + pixel_size * 0.5;
    return image.eval(pixel);
}
