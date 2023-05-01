use geng::prelude::*;
use geng::ui::*;

pub fn slider<'a, T: Float + 'a>(
    cx: &'a Controller,
    name: impl AsRef<str> + 'a,
    value: &mut T,
    range: RangeInclusive<f64>,
    font: Rc<geng::Font>,
    text_size: f32,
) -> impl geng::ui::Widget + 'a {
    let slider = Slider::new(cx, value.as_f32().into(), range);
    if let Some(change) = slider.get_change() {
        *value = T::from_f32(change as f32);
    }

    let text = geng::ui::Text::new(format!("{value:.2}"), font.clone(), text_size, Rgba::WHITE);

    geng::ui::column![
        geng::ui::Text::new(name, font, text_size, Rgba::WHITE),
        geng::ui::row![
            slider
                .fixed_size(vec2(text_size * 5.0, text_size).map(|x| x as f64))
                .padding_right(f64::from(text_size) * 0.5),
            text,
        ]
    ]
}

pub struct Slider<'a> {
    cx: &'a Controller,
    sense: &'a mut Sense,
    pos: &'a mut Option<Aabb2<f64>>,
    tick_radius: &'a mut f32,
    value: f64,
    range: RangeInclusive<f64>,
    change: RefCell<&'a mut Option<f64>>,
}

impl<'a> Slider<'a> {
    const ANIMATION_SPEED: f32 = 5.0;

    pub fn new(cx: &'a Controller, value: f64, range: RangeInclusive<f64>) -> Self {
        Slider {
            cx,
            sense: cx.get_state(),
            tick_radius: cx.get_state(),
            pos: cx.get_state(),
            value,
            range,
            change: RefCell::new(cx.get_state()),
        }
    }

    pub fn get_change(&self) -> Option<f64> {
        self.change.borrow_mut().take()
    }
}

impl<'a> Widget for Slider<'a> {
    fn sense(&mut self) -> Option<&mut Sense> {
        Some(self.sense)
    }
    fn update(&mut self, delta_time: f64) {
        let target_tick_radius = if self.sense.is_hovered() || self.sense.is_captured() {
            1.0 / 2.0
        } else {
            1.0 / 6.0
        };
        *self.tick_radius += (target_tick_radius - *self.tick_radius)
            .clamp_abs(Self::ANIMATION_SPEED * delta_time as f32);
    }
    fn draw(&mut self, cx: &mut DrawContext) {
        *self.pos = Some(cx.position);
        let draw2d = cx.draw2d;
        let position = cx.position.map(|x| x as f32);
        let line_width = position.height() / 3.0;
        let value_position = if self.range.end() == self.range.start() {
            *self.tick_radius
        } else {
            *self.tick_radius
                + ((self.value - *self.range.start()) / (*self.range.end() - *self.range.start()))
                    as f32
                    * (position.width() - line_width)
        };
        draw2d.draw2d(
            cx.framebuffer,
            &geng::PixelPerfectCamera,
            &draw2d::Quad::new(
                Aabb2::from_corners(
                    position.bottom_left()
                        + vec2(value_position, (position.height() - line_width) / 2.0),
                    position.top_right()
                        - vec2(line_width / 2.0, (position.height() - line_width) / 2.0),
                ),
                cx.theme.usable_color,
            ),
        );
        draw2d.draw2d(
            cx.framebuffer,
            &geng::PixelPerfectCamera,
            &draw2d::Ellipse::circle(
                position.top_right() - vec2(line_width / 2.0, position.height() / 2.0),
                line_width / 2.0,
                cx.theme.usable_color,
            ),
        );
        draw2d.draw2d(
            cx.framebuffer,
            &geng::PixelPerfectCamera,
            &draw2d::Quad::new(
                Aabb2::from_corners(
                    position.bottom_left()
                        + vec2(line_width / 2.0, (position.height() - line_width) / 2.0),
                    position.bottom_left()
                        + vec2(value_position, (position.height() + line_width) / 2.0),
                ),
                cx.theme.hover_color,
            ),
        );
        draw2d.draw2d(
            cx.framebuffer,
            &geng::PixelPerfectCamera,
            &draw2d::Ellipse::circle(
                position.bottom_left() + vec2(line_width / 2.0, position.height() / 2.0),
                line_width / 2.0,
                cx.theme.hover_color,
            ),
        );
        draw2d.draw2d(
            cx.framebuffer,
            &geng::PixelPerfectCamera,
            &draw2d::Ellipse::circle(
                position.bottom_left() + vec2(value_position, position.height() / 2.0),
                *self.tick_radius * position.height(),
                cx.theme.hover_color,
            ),
        );
    }
    fn handle_event(&mut self, event: &geng::Event) {
        let aabb = match *self.pos {
            Some(pos) => pos,
            None => return,
        };
        if self.sense.is_captured() {
            if let geng::Event::MouseDown { position, .. }
            | geng::Event::MouseMove { position, .. } = &event
            {
                let position = position.x - aabb.min.x;
                let new_value = *self.range.start()
                    + ((position - aabb.height() / 6.0) / (aabb.width() - aabb.height() / 3.0))
                        .clamp(0.0, 1.0)
                        * (*self.range.end() - *self.range.start());
                **self.change.borrow_mut() = Some(new_value);
            }
        }
    }

    fn calc_constraints(&mut self, _children: &ConstraintsContext) -> Constraints {
        Constraints {
            min_size: vec2(1.0, 1.0) * self.cx.theme().text_size as f64,
            flex: vec2(1.0, 0.0),
        }
    }
}
