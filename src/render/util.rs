use super::*;

pub fn new_texture(geng: &Geng) -> ugli::Texture {
    ugli::Texture::new_with(geng.ugli(), vec2(1, 1), |_| Rgba::BLACK)
}

pub fn attach_texture<'a>(texture: &'a mut ugli::Texture, geng: &Geng) -> ugli::Framebuffer<'a> {
    ugli::Framebuffer::new_color(geng.ugli(), ugli::ColorAttachment::Texture(texture))
}

pub fn update_texture_size(texture: &mut ugli::Texture, size: vec2<usize>, geng: &Geng) {
    if texture.size() != size {
        *texture = ugli::Texture::new_with(geng.ugli(), size, |_| Rgba::BLACK);
        texture.set_filter(ugli::Filter::Nearest);
    }
}

pub fn draw_collider(
    collider: &Collider,
    color: Rgba<f32>,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
    camera: &Camera2d,
) {
    let rotation = collider.rotation.as_radians();
    let collider = collider.raw().map(Coord::as_f32);
    let center = collider.center();
    geng.draw2d().draw2d(
        framebuffer,
        camera,
        &draw2d::Quad::new(collider, color)
            .translate(-center)
            .rotate(rotation)
            .translate(center),
    );
}

pub fn unit_quad() -> Vec<Vertex> {
    [(0, 0), (1, 0), (1, 1), (0, 1)]
        .into_iter()
        .map(|(x, y)| {
            let a_uv = vec2(x as f32, y as f32);
            let a_pos = a_uv * 2.0 - vec2(1.0, 1.0);
            Vertex { a_pos, a_uv }
        })
        .collect()
}

pub fn collider_geometry(collider: &Collider) -> Vec<Vertex> {
    let uvs = [(0, 0), (1, 0), (1, 1), (0, 1)].map(|(x, y)| vec2(x as f32, y as f32));
    collider
        .vertices()
        .into_iter()
        .zip(uvs)
        .map(|(pos, uv)| Vertex {
            a_pos: pos.map(Coord::as_f32),
            a_uv: uv,
        })
        .collect()
}

pub fn draw_simple(
    vertices: impl ugli::VertexDataSource,
    uniforms: impl ugli::Uniforms,
    camera: &impl geng::AbstractCamera2d,
    program: &ugli::Program,
    framebuffer: &mut ugli::Framebuffer,
) {
    ugli::draw(
        framebuffer,
        program,
        ugli::DrawMode::TriangleFan,
        vertices,
        (
            uniforms,
            camera.uniforms(framebuffer.size().map(|x| x as f32)),
        ),
        ugli::DrawParameters {
            blend_mode: Some(ugli::BlendMode::straight_alpha()),
            ..default()
        },
    )
}
