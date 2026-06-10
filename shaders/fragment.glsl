#version 150

in vec3 v_normal;
in vec3 v_position;

out vec4 color;
vec3 light = vec3(0.0, 1.0, 0.0);

const vec3 ambient_color = vec3(0.3, 0.0, 0.3); // the light the object receives
const vec3 diffuse_color = vec3(0.7, 0.0, 0.7); // the light the object emits
const vec3 specular_color = vec3(1.0, 1.0, 1.0); // the light reflected in the screen

void main(){
  float diffuse = max(dot(normalize(v_normal) , normalize(light)), 0.0);
  
  vec3 camera_direction = normalize(-v_position);
  vec3 half_direction = normalize(normalize(light) + camera_direction);
  float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

  color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
}
