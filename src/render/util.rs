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
