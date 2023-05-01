uniform mat3 u_model_matrix;
uniform mat3 u_view_matrix;
uniform mat3 u_projection_matrix;

varying vec2 v_pos;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;

void main() {
    v_pos = a_pos;
    vec3 pos = u_projection_matrix * u_view_matrix * u_model_matrix * vec3(a_pos, 1.0);
    gl_Position = vec4(pos.xy, 0.0, pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
uniform sampler2D u_visibility_texture;

void main() {
    vec4 vis_color = texture2D(u_visibility_texture, v_pos * 0.5 + 0.5);
    vec4 color = vis_color * vec4(1.0, 0.0, 0.0, 1.0);
    gl_FragColor = color;
}
#endif