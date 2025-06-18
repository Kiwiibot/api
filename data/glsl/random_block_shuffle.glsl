uniform shader image;
uniform float2 image_size;
uniform int block_map[36];

half4 main(float2 coord) {
    float2 block_size = image_size / float2(6.0, 6.0);

    int bx = int(floor(coord.x / block_size.x));
    int by = int(floor(coord.y / block_size.y));
    int current_idx = by * 6 + bx;

    int src_idx = 0;
    for (int i = 0; i < 36; ++i) {
        if (i == current_idx) {
            src_idx = block_map[i];
        }
    }

    float src_bx = mod(float(src_idx), 6.0);
    float src_by = floor(float(src_idx) / 6.0);

    float2 src_coord = float2(src_bx * block_size.x + mod(coord.x, block_size.x), src_by * block_size.y + mod(coord.y, block_size.y));

    return image.eval(src_coord);
}
