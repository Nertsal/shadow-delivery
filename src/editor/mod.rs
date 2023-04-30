use std::path::PathBuf;

use crate::{
    model::*,
    render::{EditorRender, RenderCache},
};

use super::*;

pub struct Editor {
    geng: Geng,
    assets: Rc<Assets>,
    render: EditorRender,
    render_cache: RenderCache,
    world: World,
    level_path: PathBuf,
    mode: EditorMode,
}

#[derive(Debug, Clone, Copy)]
pub enum EditorMode {
    Spawn,
    Waypoint,
    Obstacle,
}

impl Editor {
    pub fn new(geng: &Geng, assets: &Rc<Assets>, level: Level, level_path: PathBuf) -> Self {
        let world = World::new(level);
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            render: EditorRender::new(geng, assets),
            render_cache: RenderCache::calculate(&world, geng, assets),
            world,
            level_path,
            mode: EditorMode::Spawn,
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        #[cfg(not(target = "wasm32"))]
        {
            let writer = std::io::BufWriter::new(std::fs::File::create(&self.level_path)?);
            serde_json::to_writer_pretty(writer, &self.world.level)?;
        }
        Ok(())
    }
}

impl geng::State for Editor {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.render
            .draw(&self.world, &self.mode, &self.render_cache, framebuffer);

        let framebuffer_size = framebuffer.size().map(|x| x as f32);
        self.geng.draw2d().draw2d(
            framebuffer,
            &geng::PixelPerfectCamera,
            &draw2d::Text::unit(
                &**self.geng.default_font(),
                format!("Mode: {:?}", self.mode),
                Rgba::WHITE,
            )
            .scale_uniform(20.0)
            .align_bounding_box(vec2(0.0, 1.0))
            .translate(vec2(0.05, 0.95) * framebuffer_size),
        );
    }

    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown { key } => match key {
                geng::Key::S if self.geng.window().is_key_pressed(geng::Key::LCtrl) => {
                    let _ = self.save();
                }
                geng::Key::Num1 => {
                    self.mode = EditorMode::Spawn;
                }
                geng::Key::Num2 => {
                    self.mode = EditorMode::Waypoint;
                }
                geng::Key::Num3 => {
                    self.mode = EditorMode::Obstacle;
                }
                _ => {}
            },
            _ => {}
        }
    }
}

pub fn run(geng: &Geng) -> impl Future<Output = impl geng::State> {
    let geng = geng.clone();
    async move {
        let assets: Assets = geng::Load::load(geng.asset_manager(), &run_dir().join("assets"))
            .await
            .expect("Failed to load assets");

        let level_path = run_dir().join("assets").join("level.json");
        let level: model::Level = file::load_json(&level_path)
            .await
            .expect("Failed to load level");

        Editor::new(&geng, &Rc::new(assets), level, level_path)
    }
}
