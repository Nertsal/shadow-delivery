use super::*;

#[derive(geng::Load)]
pub struct Assets {
    pub shaders: Shaders,
    pub sprites: Sprites,
}

#[derive(Deref)]
pub struct Texture {
    #[deref]
    texture: Rc<ugli::Texture>,
    normal: Option<Rc<ugli::Texture>>,
}

#[derive(geng::Load)]
pub struct Shaders {
    pub color: ugli::Program,
    pub texture: ugli::Program,
    // pub texture_mask: ugli::Program,
    pub global_light: ugli::Program,
    pub spotlight: ugli::Program,
    pub point_light_shadow_map: ugli::Program,
    pub normal_map: ugli::Program,
    pub normal_texture: ugli::Program,
}

#[derive(geng::Load)]
pub struct Sprites {
    pub car: Texture,
    pub bike: Texture,
}

impl Texture {
    pub fn texture(&self) -> &ugli::Texture {
        self.texture.deref()
    }
    pub fn normal(&self) -> Option<&ugli::Texture> {
        self.normal.as_deref()
    }
}

impl geng::Load for Texture {
    fn load(manager: &geng::Manager, path: &std::path::Path) -> geng::asset::Future<Self> {
        let path = path.to_owned();
        let manager = manager.clone();
        async move {
            let mut texture = ugli::Texture::load(&manager, &path).await?;
            texture.set_filter(ugli::Filter::Nearest);
            let texture = Rc::new(texture);
            let name = path.file_stem().unwrap().to_str().unwrap();
            let normal_path = path.with_file_name(format!("{name}_normal.png"));
            let normal = util::report_warn(
                async {
                    let mut texture = ugli::Texture::load(&manager, &normal_path).await?;
                    texture.set_filter(ugli::Filter::Nearest);
                    Result::<_, anyhow::Error>::Ok(Rc::new(texture))
                }
                .await,
                format!("Failed to load normals for {name}"),
            )
            .ok();
            Ok(Self { texture, normal })
        }
        .boxed_local()
    }
    const DEFAULT_EXT: Option<&'static str> = Some("png");
}
