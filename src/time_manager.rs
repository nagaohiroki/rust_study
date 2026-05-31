use std::time::Instant;
pub struct TimeManager {
    start_time: std::time::Instant,
    last_time: std::time::Instant,
    delta_time: f32,
    frame_count: i32,
    time_scale: f32,
    fps_timer: f32,
    current_fps: f32,
}
impl TimeManager {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_time: now,
            delta_time: 0.0,
            time_scale: 1.0,
            fps_timer: 0.0,
            frame_count: 0,
            current_fps: 0.0,
        }
    }
    pub fn update(&mut self) {
        let now = Instant::now();
        let raw_delta = now.duration_since(self.last_time).as_secs_f32();
        self.last_time = now;
        self.delta_time = raw_delta * self.time_scale;
        self.frame_count += 1;
        self.fps_timer += raw_delta;
        if self.fps_timer >= 1.0 {
            self.current_fps = self.frame_count as f32 / self.fps_timer;
            self.frame_count = 0;
            self.fps_timer = 0.0;
            println!(
                "FPS:{:.1}, since:{:0.1}",
                self.current_fps,
                self.time_since_start()
            );
        }
    }
    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }
    pub fn time_since_start(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }
}
