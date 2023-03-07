pub struct Player {
    name: String,
    score: u32,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            score: 0,
        }
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_score(&self) -> u32 {
        self.score
    }
    pub fn increment_score(&mut self) {
        self.score += 1;
    }
}
