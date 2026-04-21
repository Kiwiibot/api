uniform shader image;
uniform float angle;
uniform float2 canvas_size;
uniform float2 image_size;

const float PI = 3.14159265359;

mat3 rotate_y(float a) {
  float s = sin(a);
  float c = cos(a);
  return mat3(c, 0, s, 0, 1, 0, -s, 0, c);
}

vec2 intersect_sphere(vec3 ro, vec3 rd, float r) {
  float b = dot(ro, rd);
  float c = dot(ro, ro) - r * r;
  float h = b * b - c;
  if (h < 0.0) {
    return vec2(-1.0);
  }
  float sqrt_h = sqrt(h);
  return vec2(-b - sqrt_h, -b + sqrt_h);
}

vec2 get_sphere_uv(vec3 p) {
  p = normalize(p);
  float u = 0.5 + atan(p.z, p.x) / (2.0 * PI);
  float v = 0.5 + asin(p.y) / PI;
  return vec2(u, v);
}

half4 main(vec2 coord) {
  vec2 uv = (2.0 * coord - canvas_size.xy) / canvas_size.y;

  vec3 ro = vec3(0.0, 0.0, 3.5);
  vec3 rd = normalize(vec3(uv, -2.0));

  mat3 rot = rotate_y(angle);
  ro = rot * ro;
  rd = rot * rd;

  float radius = 1.2;

  vec2 t = intersect_sphere(ro, rd, radius);
  float t_hit = -1.0;

  if (t.x > 0.0) {
    vec3 p1 = ro + rd * t.x;
    if (p1.x >= 0.0) {
      t_hit = t.x;
    }
  }

  if (t_hit < 0.0 && t.y > 0.0) {
    vec3 p2 = ro + rd * t.y;
    if (p2.x >= 0.0) {
      t_hit = t.y;
    }
  }

  if (t_hit > 0.0) {
    vec3 pos = ro + rd * t_hit;
    vec3 normal = normalize(pos);
    float facing = dot(rd, normal);

    vec2 tex_uv = get_sphere_uv(pos);
    vec2 tex_coords = tex_uv * image_size;
    half4 tex_color = image.eval(tex_coords);

    if (facing < 0.0) {
      return tex_color;
    } else {
      return tex_color * half4(0.7, 0.7, 0.7, 1.0);
    }
  }

  return half4(0.0);
}
