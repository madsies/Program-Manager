pub struct ProgramManager {
    pub count: i32,
    pub status: String,
}

impl ProgramManager {
    pub fn new() -> Self {
        Self { count: 0, status: "Ready".into() }
    }

    pub fn increment(&mut self) {
        self.count += 1;
        self.status = format!("Clicked {} times", self.count);
    }

    pub fn double(&mut self) {
        self.count *= 2;
        self.status = format!("Doubled to {}", self.count);
    }
}