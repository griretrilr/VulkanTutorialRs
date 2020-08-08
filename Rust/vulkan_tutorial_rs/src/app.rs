pub struct App {}

impl App {
    pub fn new() -> App {
        App {}
    }

    pub fn run(&mut self) {
        self.init_vulkan();
        self.main_loop();
        self.cleanup();
    }

    fn init_vulkan(&mut self) {}

    fn main_loop(&mut self) {}

    fn cleanup(&mut self) {}
}
