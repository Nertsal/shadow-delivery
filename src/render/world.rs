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
        self.draw_background(world, framebuffer);
        self.draw_props(world, framebuffer, normal_framebuffer);
        self.draw_obstacles(world, framebuffer, normal_framebuffer);
        self.draw_lamps(world, framebuffer, normal_framebuffer);
        self.draw_waypoints(world, framebuffer, normal_framebuffer);
        if world.player.health > Health::ZERO {
            self.draw_player(world, framebuffer, normal_framebuffer);
        }
        self.draw_particles(world, framebuffer, normal_framebuffer);
    }

    pub fn draw_background(&mut self, world: &World, framebuffer: &mut ugli::Framebuffer) {
        let framebuffer_size = framebuffer.size().map(|x| x as f32);
        let fov = world.camera.fov;
        let scale = vec2(framebuffer_size.aspect() * fov, fov);
        let matrix = mat3::translate(world.camera.center)
            * mat3::rotate(world.camera.rotation)
            * mat3::scale(scale);
        let geometry = [(-1, -1), (1, -1), (1, 1), (-1, 1)]
            .into_iter()
            .map(|(x, y)| draw2d::Vertex {
                a_pos: vec2(x as f32, y as f32),
            })
            .collect();
        let geometry = ugli::VertexBuffer::new_dynamic(self.geng.ugli(), geometry);
        ugli::draw(
            framebuffer,
            &self.assets.shaders.background,
            ugli::DrawMode::TriangleFan,
            &geometry,
            (
                ugli::uniforms! {
                    u_model_matrix: matrix,
                    u_texture: self.assets.sprites.props.bricks.texture(),
                },
                world.camera.uniforms(framebuffer.size().map(|x| x as f32)),
            ),
            ugli::DrawParameters {
                blend_mode: Some(ugli::BlendMode::straight_alpha()),
                ..default()
            },
        );
    }

    pub fn draw_props(
        &mut self,
        world: &World,
        framebuffer: &mut ugli::Framebuffer,
        normal_framebuffer: &mut ugli::Framebuffer,
    ) {
        #[derive(StructQuery)]
        struct PropRef<'a> {
            collider: &'a Collider,
            prop: &'a PropType,
        }
        for item in query_prop_ref!(world.level.props).values() {
            let texture = self.assets.sprites.props.get(item.prop).unwrap();
            self.draw_simple(
                item.collider,
                texture,
                &world.camera,
                framebuffer,
                normal_framebuffer,
            );
        }
    }

    pub fn draw_particles(
        &mut self,
        world: &World,
        framebuffer: &mut ugli::Framebuffer,
        _normal_framebuffer: &mut ugli::Framebuffer,
    ) {
        #[derive(StructQuery)]
        struct ParticleRef<'a> {
            position: &'a vec2<Coord>,
            lifetime: &'a Time,
            radius: &'a Coord,
            color: &'a Color,
            text: &'a Option<String>,
        }
        for particle in query_particle_ref!(world.particles).values() {
            let pos = particle.position.map(Coord::as_f32);
            let time = Time::new(0.2);
            let radius = ((*particle.lifetime).min(time) / time * *particle.radius).as_f32();
            if let Some(text) = particle.text {
                self.geng.draw2d().draw2d(
                    framebuffer,
                    &world.camera,
                    &draw2d::Text::unit(&**self.geng.default_font(), text, *particle.color)
                        .scale_uniform(radius)
                        .translate(pos),
                );
            } else {
                self.geng.draw2d().draw2d(
                    framebuffer,
                    &world.camera,
                    &draw2d::Ellipse::circle(pos, radius, *particle.color),
                );
            }
        }
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
        let unit_geometry = ugli::VertexBuffer::new_dynamic(self.geng.ugli(), unit_quad());
        for item in query_obstacle_ref!(world.obstacles).values() {
            let texture = if item.lights.is_empty() {
                // Building
                let scale_matrix = mat3::scale(item.collider.size().map(Coord::as_f32) / 2.0);
                let matrix = mat3::translate(item.collider.pos().map(Coord::as_f32))
                    * mat3::rotate(item.collider.rotation.as_radians())
                    * scale_matrix;
                ugli::draw(
                    framebuffer,
                    &self.assets.shaders.building,
                    ugli::DrawMode::TriangleFan,
                    &unit_geometry,
                    (
                        ugli::uniforms! {
                            u_scale_matrix: scale_matrix,
                            u_model_matrix: matrix,
                            u_outside_color: Rgba::opaque(0.4, 0.3, 0.35),
                            u_inside_color: Rgba::BLACK,
                        },
                        world.camera.uniforms(framebuffer.size().map(|x| x as f32)),
                    ),
                    ugli::DrawParameters {
                        blend_mode: Some(ugli::BlendMode::straight_alpha()),
                        ..default()
                    },
                );
                continue;
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
        for item in query_path_ref!(world.obstacles).values() {
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

        let obstacles = query_collider_ref!(world.obstacles);
        let waypoints = query_collider_ref!(world.level.waypoints);
        let colliders = obstacles
            .values()
            .map(|item| (item, Rgba::new(0.3, 0.3, 0.3, 0.5)))
            .chain(
                waypoints
                    .get(world.active_waypoint)
                    .map(|item| (item, Rgba::new(0.0, 1.0, 1.0, 0.5))),
            )
            .chain((world.player.health > Health::ZERO).then(|| {
                (
                    ColliderRef {
                        collider: &world.player.collider,
                    },
                    Rgba::new(0.0, 1.0, 0.0, 0.5),
                )
            }));

        for (item, color) in colliders {
            draw_collider(item.collider, color, &self.geng, framebuffer, &world.camera);
        }
    }
}
