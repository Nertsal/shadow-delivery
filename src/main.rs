use ecs::prelude::*;
use geng::prelude::*;

mod assets;
mod editor;
mod game;
mod model;
mod render;

use assets::Assets;

#[derive(clap::Parser)]
struct Args {
    #[clap(long)]
    editor: bool,
}

fn main() {
    let args: Args = clap::Parser::parse();

    logger::init();
    geng::setup_panic_handler();

    let geng = Geng::new_with(geng::ContextOptions {
        title: "Shadow Delivery".to_string(),
        ..default()
    });

    if args.editor {
        geng.clone().run_loading(editor::run(&geng))
    } else {
        geng.clone().run_loading(game::run(&geng))
    };
}
