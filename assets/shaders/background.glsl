uniform mat3 u_model_matrix;
uniform mat3 u_view_matrix;
uniform mat3 u_projection_matrix;

varying vec2 v_pos;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;

void main() {
    vec3 world_pos = u_model_matrix * vec3(a_pos, 1.0);
    v_pos = world_pos.xy / world_pos.z;
    vec3 pos = u_projection_matrix * u_view_matrix * world_pos;
    gl_Position = vec4(pos.xy, 0.0, pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
uniform sampler2D u_texture;

void main() {
    vec2 uv = fract(v_pos * 0.5);
    gl_FragColor = texture2D(u_texture, uv);
}
#endif
