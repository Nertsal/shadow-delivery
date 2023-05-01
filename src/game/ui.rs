use geng::ui::*;

use super::*;

impl Game {
    pub fn ui<'a>(&'a mut self, cx: &'a Controller) -> Box<dyn Widget + 'a> {
        if let Some(time) = self.world.death_time {
            self.death_ui(time, cx)
        } else {
            self.game_ui(cx)
        }
    }

    fn death_ui<'a>(&'a mut self, death_time: Time, cx: &'a Controller) -> Box<dyn Widget + 'a> {
        let framebuffer_size = self.framebuffer_size.map(|x| x as f32);
        let font = self.geng.default_font().clone();
        let text_size = 50.0;
        let text_color = Rgba::WHITE;

        let score = geng::ui::Text::new(
            format!("Score: {}", self.world.player.score),
            font.clone(),
            text_size,
            text_color,
        );

        let time = geng::ui::Text::new(
            format!("Time: {:.0}s", death_time),
            font,
            text_size,
            text_color,
        );

        let replay = geng::ui::Button::new(cx, "Try Again");
        if replay.was_clicked() {
            self.reset = true;
        }

        geng::ui::column![
            score.padding_bottom(text_size.into()),
            time.padding_bottom(text_size.into()),
            replay
                .fixed_size(vec2(text_size * 5.0, text_size).map(f64::from))
                .padding_bottom(text_size.into()),
        ]
        .align(vec2(0.3, 0.5))
        .uniform_padding(f64::from(framebuffer_size.y) * 0.1)
        .boxed()
    }

    fn game_ui<'a>(&mut self, _cx: &'a Controller) -> Box<dyn Widget + 'a> {
        let framebuffer_size = self.framebuffer_size.map(|x| x as f32);
        let font = self.geng.default_font().clone();

        let color = Rgba::lerp(Rgba::GREEN, Rgba::RED, self.player_visibilty);
        let visibility = geng::ui::Text::new(
            format!("Visibility: {:.0}%", self.player_visibilty * 100.0),
            font.clone(),
            30.0,
            color,
        )
        .align(vec2(0.5, 0.9))
        .fixed_size(framebuffer_size.map(|x| x.into()) * 0.1)
        .align(vec2(0.0, 0.0))
        .padding_left(framebuffer_size.y as f64 * 0.1);

        let color = Rgba::lerp(
            Rgba::RED,
            Rgba::GREEN,
            self.world.player.health.as_f32() / 100.0,
        );
        let health = geng::ui::Text::new(
            format!("Health: {:.0}", self.world.player.health.as_f32()),
            font.clone(),
            30.0,
            color,
        )
        .align(vec2(0.5, 0.5))
        .fixed_size(framebuffer_size.map(|x| x.into()) * 0.1)
        .align(vec2(0.0, 0.0))
        .padding_left(framebuffer_size.y as f64 * 0.1);

        let score = geng::ui::Text::new(
            format!("Score: {}", self.world.player.score),
            font,
            50.0,
            Rgba::WHITE,
        )
        .fixed_size(framebuffer_size.map(|x| x.into()) * 0.1)
        .align(vec2(0.5, 1.0));

        geng::ui::stack![visibility, health, score].boxed()
    }
}
