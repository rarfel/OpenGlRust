#version 430

in vec3 v_normal;
in vec3 v_position;
//in vec2 v_tex_coords;

out vec4 color;
vec3 light = vec3(-0.4, 0.6, -1.0);

uniform sampler2D tex;
uniform sampler2D normal_tex;

vec3 specular_color = vec3(0.4, 0.5, 0.8); // the light reflected in the screen

mat3 cotangent_frame(vec3 normal, vec3 pos, vec2 uv) {
    vec3 dp1 = dFdx(pos);
    vec3 dp2 = dFdy(pos);
    vec2 duv1 = dFdx(uv);
    vec2 duv2 = dFdy(uv);

    vec3 dp2perp = cross(dp2, normal);
    vec3 dp1perp = cross(normal, dp1);
    vec3 T = dp2perp * duv1.x + dp1perp * duv2.x;
    vec3 B = dp2perp * duv1.y + dp1perp * duv2.y;

    float invmax = inversesqrt(max(dot(T, T), dot(B, B)));
    return mat3(T * invmax, B * invmax, normal);
}

void main(){
  //vec3 diffuse_color = texture(tex, v_tex_coords).rgb; // the light the object emits
  //vec3 ambient_color = diffuse_color * 0.1; // the light the object receives
  vec3 diffuse_color = vec3(0.5, 0.5, 0.5);
  vec3 ambient_color = vec3(0.4, 0.4, 0.4);

  vec3 normal_unit = normalize(v_normal);
  //vec3 normal_map = texture(normal_tex, v_tex_coords).rgb;
  //mat3 tbn = cotangent_frame(normal_unit,  v_position, v_tex_coords);
  //vec3 real_normal = normalize(tbn * -(normal_map * 2.0 - 1.0));

  float diffuse = max(dot(normal_unit , normalize(light)), 0.0);
  
  vec3 camera_direction = normalize(-v_position);
  vec3 half_direction = normalize(normalize(light) + camera_direction);
  float specular = pow(max(dot(half_direction, normal_unit), 0.0), 16.0);

  color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
}
