#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Timing {
    Linear,
    EaseInOut,
}

#[derive(Clone)]
pub struct Animation {
    pub curve: lyon::geom::CubicBezierSegment<f32>,
    pub duration: f32,
    pub began: std::time::Instant,
    pub attribute: String,
}

impl Animation {
    fn tick(&self) -> (f32, f32) {
        let mut progress = std::time::Instant::now()
            .duration_since(self.began)
            .as_secs_f32()
            / self.duration;
        progress = progress.min(1.0);
        let nxt_val = self.curve.sample(progress).y;
        (nxt_val, progress)
    }
}
