use super::*;

impl World {
    pub fn calculate_static_light_geometry(&self) -> Vec<render::NormalVertex> {
        #[derive(StructQuery)]
        struct ObstacleRef<'a> {
            collider: &'a Collider,
            path: &'a Option<Path>,
        }
        query_obstacle_ref!(self.level.obstacles)
            .iter()
            .filter(|(_, item)| item.path.is_none())
            .flat_map(|(_, item)| collider_light_geometry(item.collider))
            .collect()
    }

    pub fn calculate_dynamic_light_geometry(&self) -> Vec<render::NormalVertex> {
        #[derive(StructQuery)]
        struct ObstacleRef<'a> {
            collider: &'a Collider,
            path: &'a Option<Path>,
        }
        query_obstacle_ref!(self.obstacles)
            .iter()
            .filter(|(_, item)| item.path.is_some())
            .flat_map(|(_, item)| collider_light_geometry(item.collider))
            .collect()
    }

    pub fn calculate_normal_geometry(
        &self,
        geng: &Geng,
        _assets: &Assets,
    ) -> ugli::VertexBuffer<render::NormalVertex> {
        #[derive(StructQuery)]
        struct ObstacleRef<'a> {
            collider: &'a Collider,
            path: &'a Option<Path>,
        }
        let normals = [(-1, -1), (1, -1), (1, 1), (-1, 1)]
            .map(|(x, y)| vec2(x as f32, y as f32).normalize() * 0.3);
        let query = query_obstacle_ref!(self.obstacles);
        let geometry = query
            .values()
            .filter(|item| item.path.is_none())
            .flat_map(|item| {
                let aabb = item.collider.raw().map(Coord::as_f32);
                let center = aabb.center();
                let rotation = item.collider.rotation.as_radians().as_f32();
                let vs = aabb
                    .corners()
                    .into_iter()
                    .zip(normals)
                    .map(move |(p, n)| {
                        let a_pos = (p - center).rotate(rotation) + center;
                        let a_normal = n.rotate(rotation);
                        render::NormalVertex { a_pos, a_normal }
                    })
                    .collect::<Vec<_>>();
                [vs[0], vs[1], vs[2], vs[0], vs[2], vs[3]]
            })
            .collect();
        ugli::VertexBuffer::new_dynamic(geng.ugli(), geometry)
    }
}

fn collider_light_geometry(collider: &Collider) -> impl Iterator<Item = render::NormalVertex> {
    let vs = collider.vertices().map(|v| v.map(Coord::as_f32));
    let sides = [
        (vs[0], vs[1]),
        (vs[1], vs[2]),
        (vs[2], vs[3]),
        (vs[3], vs[0]),
    ];

    sides.into_iter().flat_map(|(a, b)| {
        let a_normal = (a - b).rotate_90().normalize_or_zero();
        let [a, b] = [a, b].map(|v| render::NormalVertex { a_pos: v, a_normal });
        let [a1, b1] = [a, b].map(|mut v| {
            v.a_normal = vec2::ZERO;
            v
        });
        [b1, a1, a, b1, a, b]
    })
}
