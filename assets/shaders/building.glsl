uniform mat3 u_scale_matrix;
uniform mat3 u_model_matrix;
uniform mat3 u_view_matrix;
uniform mat3 u_projection_matrix;

varying vec2 v_pos;
varying vec2 v_size;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;

void main() {
    vec3 size = u_scale_matrix * vec3(1.0);
    v_size = size.xy / size.z;

    vec3 scaled_pos = u_scale_matrix * vec3(a_pos, 1.0);
    v_pos = scaled_pos.xy / scaled_pos.z;

    vec3 pos = u_projection_matrix * u_view_matrix * u_model_matrix * vec3(a_pos, 1.0);
    gl_Position = vec4(pos.xy, 0.0, pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
uniform vec4 u_outside_color;
uniform vec4 u_inside_color;

void main() {
    vec2 dist = v_size - abs(v_pos);
    float d = min(dist.x, dist.y);
    float t = min(d, 1.0);

    vec4 color = u_outside_color + (u_inside_color - u_outside_color) * t;

    gl_FragColor = color;
}
#endif
