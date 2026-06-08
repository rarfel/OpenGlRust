#version 150

in vec3 position;
in vec3 normal;

out vec3 v_normal;

uniform mat4 projection;
uniform mat4 matrix;

void main() {
  float scaler = 0.01;
  v_normal = transpose(inverse(mat3(matrix))) * normal;
  gl_Position = matrix * projection * vec4(position * scaler, 1.0);
}
