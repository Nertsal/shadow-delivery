use std::path::PathBuf;

use crate::{model::*, render::RenderCache};

use super::*;

mod render;

use geng::MouseButton;
use render::*;

pub struct Editor {
    geng: Geng,
    #[allow(dead_code)]
    assets: Rc<Assets>,
    render: EditorRender,
    render_cache: RenderCache,
    framebuffer_size: vec2<usize>,
    world: World,
    level_path: PathBuf,
    mode: EditorMode,
    drag: Option<Drag>,
    cursor_pos: vec2<Coord>,
}

struct Drag {
    from: vec2<Coord>,
    target: DragTarget,
}

enum DragTarget {
    Spawn,
    Waypoint(usize),
    Obstacle(usize),
    NewObstacle,
}

#[derive(Debug, Clone, Copy)]
enum EditorMode {
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
            cursor_pos: vec2::ZERO,
        }
    }

    pub fn load(&mut self) -> anyhow::Result<()> {
        #[cfg(not(target = "wasm32"))]
        {
            let reader = std::io::BufReader::new(std::fs::File::open(&self.level_path)?);
            self.world.level = serde_json::from_reader(reader)?;
            log::info!("Loaded level from {:?}", self.level_path);
        }
        Ok(())
    }

    pub fn save(&self) -> anyhow::Result<()> {
        #[cfg(not(target = "wasm32"))]
        {
            let writer = std::io::BufWriter::new(std::fs::File::create(&self.level_path)?);
            serde_json::to_writer_pretty(writer, &self.world.level)?;
            log::info!("Saved the level at {:?}", self.level_path);
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

    fn remove(&mut self, target: DragTarget) {
        match target {
            DragTarget::Waypoint(id) => {
                self.world.level.waypoints.remove(id);
            }
            DragTarget::Obstacle(id) => {
                self.world.level.obstacles.remove(id);
            }
            _ => {}
        }
    }

    fn click(&mut self, position: vec2<f64>, button: MouseButton) {
        let world_pos = self.screen_to_world(position);

        if let Some(target) = self.find_target(world_pos) {
            match button {
                MouseButton::Left => {
                    self.drag = Some(Drag {
                        from: world_pos,
                        target,
                    });
                    return;
                }
                MouseButton::Right => {
                    self.remove(target);
                    return;
                }
                MouseButton::Middle => todo!(),
            }
        }

        if !matches!(button, MouseButton::Left) {
            return;
        }

        match self.mode {
            EditorMode::Spawn => {}
            EditorMode::Waypoint => {
                let aabb = Aabb2::point(world_pos).extend_uniform(Coord::new(0.25));
                self.world.level.waypoints.insert(Waypoint {
                    collider: Collider::new(aabb),
                });
            }
            EditorMode::Obstacle => {
                self.drag = Some(Drag {
                    from: world_pos,
                    target: DragTarget::NewObstacle,
                });
            }
        }
    }

    fn update_cursor(&mut self, position: vec2<f64>) {
        let world_pos = self.screen_to_world(position);
        self.cursor_pos = world_pos;

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
                _ => {}
            }
        }
    }

    fn release(&mut self) {
        if let Some(drag) = self.drag.take() {
            if let DragTarget::NewObstacle = drag.target {
                let aabb = Aabb2::from_corners(drag.from, self.cursor_pos);
                self.world.level.obstacles.insert(Obstacle {
                    collider: Collider::new(aabb),
                    lights: default(),
                    path: default(),
                });
            }
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
        self.draw(framebuffer);

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

    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;

        let window = self.geng.window();
        let pressed = |keys: &[geng::Key]| keys.iter().any(|key| window.is_key_pressed(*key));

        let mut camera_move = vec2::ZERO;
        if pressed(&[geng::Key::W]) {
            camera_move.y += 1.0;
        }
        if pressed(&[geng::Key::S]) {
            camera_move.y -= 1.0;
        }
        if pressed(&[geng::Key::A]) {
            camera_move.x -= 1.0;
        }
        if pressed(&[geng::Key::D]) {
            camera_move.x += 1.0;
        }

        let speed = 20.0;
        self.world.camera.center += camera_move * speed * delta_time;
    }

    fn handle_event(&mut self, event: geng::Event) {
        let ctrl = self.geng.window().is_key_pressed(geng::Key::LCtrl);
        match event {
            geng::Event::KeyDown { key } => match key {
                geng::Key::S if ctrl => {
                    let _ = util::report_err(self.save());
                }
                geng::Key::L if ctrl => {
                    let _ = util::report_err(self.load());
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
            geng::Event::MouseDown { position, button } => {
                self.click(position, button);
            }
            geng::Event::MouseMove { position, .. } => {
                self.update_cursor(position);
            }
            geng::Event::MouseUp {
                button: MouseButton::Left,
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
