use super::*;

#[derive(geng::Load)]
pub struct Assets {
    pub shaders: Shaders,
    pub sprites: Sprites,
    pub sounds: Sounds,
    #[load(postprocess = "looped")]
    pub music: geng::Sound,
}

#[derive(Deref)]
pub struct Texture {
    #[deref]
    texture: Rc<ugli::Texture>,
    normal: Option<Rc<ugli::Texture>>,
}

#[derive(geng::Load)]
pub struct Sounds {
    pub bounce: geng::Sound,
    pub hurt: geng::Sound,
    pub deliver: geng::Sound,
    pub death: geng::Sound,
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
    pub visibility: ugli::Program,
    pub background: ugli::Program,
    pub health: ugli::Program,
    pub building: ugli::Program,
}

#[derive(geng::Load)]
pub struct Sprites {
    pub props: Props,
    pub car: Texture,
    pub wall: Texture,
    pub bike: Texture,
    pub target: Texture,
    pub lamp: Texture,
    #[load(postprocess = "pixel")]
    pub arrow: ugli::Texture,
}

#[derive(geng::Load)]
pub struct Props {
    pub road: Texture,
    pub road_v: Texture,
    pub road_ur: Texture,
    pub road_ul: Texture,
    pub road_rd: Texture,
    pub road_dl: Texture,
    pub bricks: Texture,
}

fn looped(sound: &mut geng::Sound) {
    sound.set_looped(true);
}

fn pixel(texture: &mut ugli::Texture) {
    texture.set_filter(ugli::Filter::Nearest);
}

impl Props {
    pub fn get(&self, prop: &model::PropType) -> Option<&Texture> {
        match prop.as_str() {
            "road" => Some(&self.road),
            "road_v" => Some(&self.road_v),
            "road_ur" => Some(&self.road_ur),
            "road_ul" => Some(&self.road_ul),
            "road_rd" => Some(&self.road_rd),
            "road_dl" => Some(&self.road_dl),
            "bricks" => Some(&self.bricks),
            _ => None,
        }
    }
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
