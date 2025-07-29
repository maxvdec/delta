use cgraph::app::WindowOptions;

pub struct Window {
    pub title: String,
    pub width: u32,
    pub height: u32,
    window: Option<cgraph::app::Window>,
    options: cgraph::app::WindowOptions,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Window {
        Window {
            title: title.to_string(),
            width,
            height,
            window: None,
            options: cgraph::app::WindowOptions::default(),
        }
    }

    pub fn without_decorations(mut self) -> Self {
        self.options = WindowOptions::no_titlebar();
        self
    }

    pub fn no_border(mut self) -> Self {
        self.options = WindowOptions::no_decorations();
        self
    }

    pub fn launch(&mut self) {
        self.window = Some(cgraph::app::Window::new(
            &self.title,
            self.width,
            self.height,
            Some(self.options.clone()),
        ));
        if let Some(window) = self.window.take() {
            window.launch();
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
        if let Some(window) = &mut self.window {
            window.set_title(&self.title);
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        if let Some(window) = &mut self.window {
            window.set_size(self.width, self.height);
        }
    }
}
