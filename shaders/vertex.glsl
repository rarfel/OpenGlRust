#version 430

in vec3 position;
in vec3 normal;
//in vec2 tex_coords;

out vec3 v_normal;
out vec3 v_position;
out vec2 v_tex_coords;

uniform mat4 projection;
uniform mat4 matrix;
uniform mat4 view;
uniform mat4 rotation;

void main() {
  float scaler = 1.0;
  //v_tex_coords = tex_coords;
  mat4 viewmatrix = (view) * (matrix * rotation);
  v_normal = transpose(inverse(mat3(viewmatrix))) * normal;
  gl_Position = projection * viewmatrix * vec4(position * scaler, 1.0);
  v_position = gl_Position.xyz / gl_Position.w;
}
