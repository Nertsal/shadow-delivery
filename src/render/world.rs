use crate::assets::Texture;

use super::*;

pub struct WorldRender {
    geng: Geng,
    #[allow(dead_code)]
    assets: Rc<Assets>,
}

impl WorldRender {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
        }
    }

    pub fn draw(
        &mut self,
        world: &World,
        framebuffer: &mut ugli::Framebuffer,
        normal_framebuffer: &mut ugli::Framebuffer,
    ) {
        self.draw_obstacles(world, framebuffer, normal_framebuffer);
        self.draw_lamps(world, framebuffer, normal_framebuffer);
        self.draw_waypoints(world, framebuffer, normal_framebuffer);
        self.draw_player(world, framebuffer, normal_framebuffer);
    }

    pub fn draw_player(
        &mut self,
        world: &World,
        framebuffer: &mut ugli::Framebuffer,
        normal_framebuffer: &mut ugli::Framebuffer,
    ) {
        self.draw_simple(
            &world.player.collider,
            &self.assets.sprites.bike,
            &world.camera,
            framebuffer,
            normal_framebuffer,
        );
    }

    pub fn draw_lamps(
        &mut self,
        world: &World,
        framebuffer: &mut ugli::Framebuffer,
        normal_framebuffer: &mut ugli::Framebuffer,
    ) {
        #[derive(StructQuery)]
        struct LampRef<'a> {
            collider: &'a Collider,
        }
        for item in query_lamp_ref!(world.level.lamps).values() {
            let texture = &self.assets.sprites.lamp;
            self.draw_simple(
                item.collider,
                texture,
                &world.camera,
                framebuffer,
                normal_framebuffer,
            );
        }
    }

    pub fn draw_obstacles(
        &mut self,
        world: &World,
        framebuffer: &mut ugli::Framebuffer,
        normal_framebuffer: &mut ugli::Framebuffer,
    ) {
        #[derive(StructQuery)]
        struct ObstacleRef<'a> {
            collider: &'a Collider,
            lights: &'a Vec<Spotlight>,
        }
        for item in query_obstacle_ref!(world.level.obstacles).values() {
            let texture = if item.lights.is_empty() {
                // House
                &self.assets.sprites.wall
            } else {
                // Car
                &self.assets.sprites.car
            };
            self.draw_simple(
                item.collider,
                texture,
                &world.camera,
                framebuffer,
                normal_framebuffer,
            );
        }
    }

    pub fn draw_waypoints(
        &mut self,
        world: &World,
        framebuffer: &mut ugli::Framebuffer,
        normal_framebuffer: &mut ugli::Framebuffer,
    ) {
        #[derive(StructQuery)]
        struct WaypointRef<'a> {
            collider: &'a Collider,
        }
        let query = query_waypoint_ref!(world.level.waypoints);
        if let Some(item) = query.get(world.active_waypoint) {
            self.draw_simple(
                item.collider,
                &self.assets.sprites.target,
                &world.camera,
                framebuffer,
                normal_framebuffer,
            );
        }
    }

    pub fn draw_simple(
        &self,
        collider: &Collider,
        texture: &Texture,
        camera: &Camera2d,
        framebuffer: &mut ugli::Framebuffer,
        normal_framebuffer: &mut ugli::Framebuffer,
    ) {
        let vertices = collider_geometry(collider);
        let vertices = ugli::VertexBuffer::new_dynamic(self.geng.ugli(), vertices);

        draw_simple(
            &vertices,
            ugli::uniforms! {
                u_model_matrix: mat3::identity(),
                u_color: Rgba::WHITE,
                u_texture: texture.texture(),
            },
            camera,
            &self.assets.shaders.texture,
            framebuffer,
        );
        if let Some(texture) = texture.normal() {
            draw_simple(
                &vertices,
                ugli::uniforms! {
                    u_model_matrix: mat3::identity(),
                    u_normal_influence: 1.0,
                    u_normal_texture: texture,
                },
                camera,
                &self.assets.shaders.normal_texture,
                normal_framebuffer,
            );
        }
    }

    pub fn draw_paths(&mut self, world: &World, framebuffer: &mut ugli::Framebuffer) {
        #[derive(StructQuery)]
        struct PathRef<'a> {
            #[query(component = "Option<Path>")]
            path: &'a Path,
        }
        for item in query_path_ref!(world.level.obstacles).values() {
            let mut points = item
                .path
                .points
                .iter()
                .map(|(_, point)| point.map(Coord::as_f32))
                .collect::<Vec<_>>();
            if let Some(&point) = points.first() {
                points.push(point);
            }
            let chain = Chain::new(points);
            self.geng.draw2d().draw2d(
                framebuffer,
                &world.camera,
                &draw2d::Chain::new(chain, 0.1, Rgba::new(0.4, 0.4, 0.4, 0.5), 2),
            );
        }
    }

    pub fn draw_hitboxes(&mut self, world: &World, framebuffer: &mut ugli::Framebuffer) {
        #[derive(StructQuery)]
        struct ColliderRef<'a> {
            collider: &'a Collider,
        }

        let obstacles = query_collider_ref!(world.level.obstacles);
        let waypoints = query_collider_ref!(world.level.waypoints);
        let colliders = obstacles
            .values()
            .map(|item| (item, Rgba::new(0.3, 0.3, 0.3, 0.5)))
            .chain(
                waypoints
                    .get(world.active_waypoint)
                    .map(|item| (item, Rgba::new(0.0, 1.0, 1.0, 0.5))),
            )
            .chain(std::iter::once((
                ColliderRef {
                    collider: &world.player.collider,
                },
                Rgba::new(0.0, 1.0, 0.0, 0.5),
            )));

        for (item, color) in colliders {
            draw_collider(item.collider, color, &self.geng, framebuffer, &world.camera);
        }
    }
}
