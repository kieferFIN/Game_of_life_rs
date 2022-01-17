use std::time::Instant;

pub struct Timer {
    moments: Vec<(Instant, &'static str)>,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            moments: Vec::new(),
        }
    }

    pub fn add(&mut self, s: &'static str) {
        self.moments.push((Instant::now(), s));
    }
    pub fn print(&self) {
        for i in 1..self.moments.len() {
            let (m, s) = self.moments[i];
            let dur = m.duration_since(self.moments[i - 1].0);
            print!("{}:{}  {}\n", dur.as_secs(), dur.subsec_millis(), s);
        }
    }
}