
uniform shader image;
uniform float time;

half4 main(float2 coord) {
    float speed = 4.0;
    float magnitude = 10.0;

    float2 offset = float2(
        sin(time * speed * 3.14159 * 2.0 + coord.y * 0.1) * magnitude,
        cos(time * speed * 3.14159 * 2.0 + coord.x * 0.1) * magnitude
    );

    return image.eval(coord + offset);
}
