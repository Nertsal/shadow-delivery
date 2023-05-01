varying vec2 v_pos;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;

void main() {
    v_pos = a_pos;
    vec3 pos = vec3(a_pos, 1.0);
    gl_Position = vec4(pos.xy, 0.0, pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
uniform float u_health;
uniform float u_time;

float smooth_step(float t) {
    if (t < 0.0) {
        return 0.0;
    }
    if (t > 1.0) {
        return 1.0;
    }
    return 3.0 * t * t - 2.0 * t * t * t;
}

void main() {
    float max = 1.41;
    float min = 0.95;

    float t = u_health * u_health;
    float d = min + (max - min) * t;

    d += sin(u_time * 6.0) * 0.05;
    
    float dist = length(v_pos);
    if (dist < d) {
        discard;
    }

    float alpha = smooth_step((dist - d) * 2.0);

    vec4 color = vec4(0.5, 0.0, 0.0, alpha);

    gl_FragColor = color;
}
#endif
