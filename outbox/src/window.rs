use cgraph::app::{CoreEvent, CoreEventReference, CoreWindowEvent, WindowOptions};

use crate::renderable::Renderable;

pub struct InteractableObject {
    pub bounds: [f32; 4], // [x, y, width, height]
    pub renderable: Box<dyn Renderable>,
}

pub struct Window {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub app_font: String,
    window: Option<cgraph::app::Window>,
    options: cgraph::app::WindowOptions,
    main_view: Box<dyn Renderable>,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Window {
        Window {
            title: title.to_string(),
            width,
            height,
            window: None,
            options: cgraph::app::WindowOptions::default(),
            main_view: Box::new(crate::component::Empty::default()),
            app_font: "Arial".to_string(),
        }
    }

    pub fn without_decorations(mut self) -> Self {
        self.options = WindowOptions::no_titlebar();
        self
    }

    pub fn without_border(mut self) -> Self {
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
        if let Some(mut window) = self.window.take() {
            let first_padding = self.main_view.get_padding();

            // Render components
            for view in self.main_view.render(
                [window.width as f32, window.height as f32],
                [0.0 + first_padding[0], 0.0 + first_padding[1]],
            ) {
                window.add_object(view);
            }

            // Clone the main_view for use in the event handler
            let main_view_clone = self.main_view.copy();

            // Add an event handler for mouse clicks to route to components
            window.on_event(
                CoreEventReference::WindowEvent,
                move |_win_window, event, _shared| {
                    if let CoreEvent::WindowEvent(CoreWindowEvent::MouseClick(_, _, state)) = event
                    {
                        let state_str = format!("{state:?}");
                        if state_str.contains("Pressed") {
                            // For now, trigger event handlers on the main view
                            // In the future, we can implement proper hit testing
                            if let Some(event_handler) = main_view_clone.get_event_handler() {
                                event_handler.handle_event(event);
                            }
                            println!("Mouse clicked!");
                        }
                    }
                },
            );

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

    pub fn set_main_view<T: Renderable + 'static>(&mut self, view: T) {
        self.main_view = Box::new(view);
    }
}
