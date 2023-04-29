use ecs::prelude::*;
use geng::prelude::*;

mod assets;
mod game;
mod model;
mod render;

use assets::Assets;

fn main() {
    let geng = Geng::new_with(geng::ContextOptions {
        title: "Shadow Delivery".to_string(),
        ..default()
    });

    let future = {
        let geng = geng.clone();
        async move {
            let assets: Assets = geng::Load::load(geng.asset_manager(), &run_dir().join("assets"))
                .await
                .expect("Failed to load assets");
            game::Game::new(&geng, &Rc::new(assets))
        }
    };

    geng.run_loading(future)
}
