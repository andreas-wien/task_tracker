pub struct CLI {
    arguments: Vec<String>,
}

impl CLI {
    pub fn new() -> Self {
        let arguments: Vec<String> = std::env::args().collect();
        CLI { arguments }
    }

    pub fn arguments(&self) -> &Vec<String> {
        &self.arguments
    }
}
