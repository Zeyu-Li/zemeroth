use std::time::Duration;

use mq::prelude::Color;

use crate::{duration_to_f32, Action, Sprite};

#[derive(Debug)]
pub struct ChangeColorTo {
    sprite: Sprite,
    from: [f32; 4], // TODO: use graphics::Color
    to: [f32; 4],   // TODO: use graphics::Color
    duration: Duration,
    progress: Duration,
}

impl ChangeColorTo {
    pub fn new(sprite: &Sprite, to: Color, duration: Duration) -> Self {
        Self {
            sprite: sprite.clone(),
            from: sprite.color().into(),
            to: to.into(),
            duration,
            progress: Duration::new(0, 0),
        }
    }
}

impl Action for ChangeColorTo {
    fn begin(&mut self) {
        self.from = self.sprite.color().into();
    }

    fn update(&mut self, mut dtime: Duration) {
        if dtime + self.progress > self.duration {
            dtime = self.duration - self.progress;
        }
        let progress_f = duration_to_f32(self.progress);
        let duration_f = duration_to_f32(self.duration);
        let k = progress_f / duration_f;
        let mut color = [0.0; 4];
        for (i, color_i) in color.iter_mut().enumerate().take(4) {
            let diff = self.to[i] - self.from[i];
            *color_i = self.from[i] + diff * k;
        }
        self.sprite.set_color(color.into());
        self.progress += dtime;
    }

    fn end(&mut self) {
        self.sprite.set_color(self.to.into());
    }

    fn duration(&self) -> Duration {
        self.duration
    }

    fn is_finished(&self) -> bool {
        self.progress >= self.duration
    }
}
