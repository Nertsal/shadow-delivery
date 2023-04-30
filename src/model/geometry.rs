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
        query_obstacle_ref!(self.level.obstacles)
            .iter()
            .filter(|(_, item)| item.path.is_some())
            .flat_map(|(_, item)| collider_light_geometry(item.collider))
            .collect()
    }

    // pub fn calculate_normal_geometry(
    //     &self,
    //     geng: &Geng,
    //     assets: &Assets,
    // ) -> ugli::VertexBuffer<render::NormalVertex> {
    //     todo!()
    // }
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
