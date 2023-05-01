uniform mat3 u_model_matrix;
uniform mat3 u_view_matrix;
uniform mat3 u_projection_matrix;

varying vec2 v_uv;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;
attribute vec2 a_uv;

void main() {
    v_uv = a_uv;
    vec3 pos = u_projection_matrix * u_view_matrix * u_model_matrix * vec3(a_pos, 1.0);
    gl_Position = vec4(pos.xy, 0.0, pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
uniform sampler2D u_texture;
uniform float u_alpha;

float rescale(float t) {
    return sqrt(t);
}

void main() {
    vec4 vis_color = texture2D(u_texture, v_uv);
    vec4 smoothed = vec4(rescale(vis_color.r), rescale(vis_color.g), rescale(vis_color.b), vis_color.a);
    float alpha = u_alpha;
    if (u_alpha < 1.0) {
        alpha *= smoothed.r;
    }
    vec4 color = smoothed * vec4(1.0, 0.0, 0.0, alpha);
    gl_FragColor = color;
}
#endif
