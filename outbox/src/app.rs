use crate::window::Window;

pub struct Application {
    pub name: String,
    pub version: String,
    pub windows: Vec<Window>,
    pub main_window: Option<Window>,
}

impl Default for Application {
    fn default() -> Self {
        Application {
            name: "Outbox".to_string(),
            version: "0.1.0".to_string(),
            windows: Vec::new(),
            main_window: None,
        }
    }
}

impl Application {
    pub fn new(name: &str, version: &str) -> Self {
        Application {
            name: name.to_string(),
            version: version.to_string(),
            windows: Vec::new(),
            main_window: None,
        }
    }

    pub fn set_main_window(&mut self, window: Window) {
        self.main_window = Some(window);
    }

    pub fn add_window(&mut self, window: Window) {
        self.windows.push(window);
    }

    pub fn run(&mut self) {
        if let Some(main_window) = &mut self.main_window.take() {
            main_window.launch();
        } else {
            eprintln!("No main window set for the application.");
        }
    }
}
