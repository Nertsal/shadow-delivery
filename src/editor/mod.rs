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
    framebuffer_size: vec2<usize>,
    world: World,
    level_path: PathBuf,
    mode: EditorMode,
    drag: Option<Drag>,
}

pub struct Drag {
    pub from: vec2<Coord>,
    pub target: DragTarget,
}

pub enum DragTarget {
    Spawn,
    Waypoint(usize),
    Obstacle(usize),
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
            framebuffer_size: vec2(1, 1),
            world,
            level_path,
            mode: EditorMode::Spawn,
            drag: None,
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

    fn screen_to_world(&self, position: vec2<f64>) -> vec2<Coord> {
        self.world
            .camera
            .screen_to_world(
                self.framebuffer_size.map(|x| x as f32),
                position.map(|x| x as f32),
            )
            .map(Coord::new)
    }

    fn click(&mut self, position: vec2<f64>) {
        let world_pos = self.screen_to_world(position);

        if let Some(target) = self.find_target(world_pos) {
            self.drag = Some(Drag {
                from: world_pos,
                target,
            });
        }
    }

    fn update_cursor(&mut self, position: vec2<f64>) {
        let world_pos = self.screen_to_world(position);

        if let Some(drag) = &mut self.drag {
            match drag.target {
                DragTarget::Spawn => {
                    self.world.level.spawn_point = world_pos;
                }
                DragTarget::Waypoint(id) => {
                    self.world
                        .level
                        .waypoints
                        .collider
                        .get_mut(id)
                        .unwrap()
                        .teleport(world_pos);
                }
                DragTarget::Obstacle(id) => {
                    self.world
                        .level
                        .obstacles
                        .collider
                        .get_mut(id)
                        .unwrap()
                        .teleport(world_pos);
                }
            }
        }
    }

    fn release(&mut self) {
        if let Some(drag) = self.drag.take() {
            // TODO
        }
    }

    fn find_target(&self, position: vec2<Coord>) -> Option<DragTarget> {
        let mut player_collider = self.world.player.collider;
        player_collider.teleport(self.world.level.spawn_point);

        #[derive(StructQuery)]
        struct ColliderRef<'a> {
            collider: &'a Collider,
        }

        let waypoints = query_collider_ref!(self.world.level.waypoints);
        let obstacles = query_collider_ref!(self.world.level.obstacles);
        let mut colliders = std::iter::once((
            DragTarget::Spawn,
            ColliderRef {
                collider: &player_collider,
            },
        ))
        .chain(
            waypoints
                .iter()
                .map(|(id, item)| (DragTarget::Waypoint(id), item)),
        )
        .chain(
            obstacles
                .iter()
                .map(|(id, item)| (DragTarget::Obstacle(id), item)),
        );

        let target = Collider::new(Aabb2::point(position).extend_uniform(Coord::new(0.01)));
        colliders
            .find(|(_, item)| item.collider.check(&target))
            .map(|(id, _)| id)
    }
}

impl geng::State for Editor {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size();
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
            geng::Event::MouseDown {
                position,
                button: geng::MouseButton::Left,
            } => {
                self.click(position);
            }
            geng::Event::MouseMove { position, .. } => {
                self.update_cursor(position);
            }
            geng::Event::MouseUp {
                button: geng::MouseButton::Left,
                ..
            } => {
                self.release();
            }
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
