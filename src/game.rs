enum Scene {
    Menu,
    Loading,
}
pub struct Game {
    pub running: bool,
    pub scene: Scene,
}

impl Game {
    pub fn new() -> Self {
        Game {
            running: true,
            scene: Scene::Menu,
        }
    }
}