use std::hash::{DefaultHasher, Hash, Hasher};

use winit::{
    dpi::PhysicalPosition,
    event::{DeviceId, Event, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{app::SharedObjects, object::primitives::Color, renderer::create_renderer};
#[cfg(target_os = "macos")]
/// Context information for the application.
pub struct Context {
    /// The name of the application.
    pub name: String,
    /// The version of the application.
    pub version: String,
    /// The description of the application.
    pub description: String,
}

/// Represents the core window events that can occur in the application.
pub enum CoreWindowEvent {
    /// The window was resized. Takes the new width and height as parameters.
    Resized(f64, f64), // x, y
    /// The window was moved. Takes the new position as parameters.
    Moved(f64, f64), // x, y
    /// The window is being closed.
    Closing,
    /// The window was destroyed.
    Destroyed,
    /// A file was dropped onto the window. Takes the file path as a parameter.
    DroppedFile(String), // path
    /// A file is being hovered over the window. Takes the file path as a parameter.
    HoveredFile(String), // path
    /// The hovering of a file was cancelled.
    HoveredFileCancelled,
    /// The window gained or lost focus. Takes a boolean indicating focus state.
    Focused(bool), // focused
    /// A character was received. Takes the character as a parameter.
    RecievedChar(char), // character
    /// A keyboard input event occurred. Takes the keyboard input as a parameter.
    KeyboardInput(winit::event::KeyboardInput), // input
    /// The modifiers (like Shift, Ctrl, etc.) were changed. Takes the modifiers state as a parameter.
    ModifierChanged(winit::event::ModifiersState), // modifiers
    /// The DPI (dots per inch) scale factor was changed. Takes the new scale factor as parameters.
    CursorMoved(u32, f64, f64), // device id, x, y
    /// The cursor entered the window. Takes the device id as a parameter.
    CursorEntered(u32), // device id
    /// The cursor left the window. Takes the device id as a parameter.
    CursorLeft(u32), // device id
    /// The mouse was scrolled. Takes the device id, scroll delta, and touch phase as parameters.
    MouseScroll(
        u32,
        winit::event::MouseScrollDelta,
        winit::event::TouchPhase,
    ), // device id, delta, phase
    /// A mouse button was clicked. Takes the device id, button, and state as parameters.
    MouseClick(u32, winit::event::MouseButton, winit::event::ElementState), // device id, button, state
    /// The touchpad was pressed. Takes the device id and pressure as parameters.
    TouchpadPressure(u32, f64), // device id, pressure
    /// The axis motion event occurred. Takes the device id, axis id, and value as parameters.
    AxisMotion(u32, winit::event::AxisId, f64), // device id, axis id, value
    /// The window theme was changed. Takes the new theme as a parameter.
    Touch(winit::event::Touch), // touch event
    /// The DPI scale factor was changed. Takes the new scale factor as parameters.
    DPIChanged(f64, f64), // scale x, scale y
    /// The window theme was changed. Takes the new theme as a parameter.
    ThemeChanged(winit::window::Theme), // theme
    /// The window is occluded. Takes a boolean indicating occlusion state.
    Occluded(bool), // occluded
    /// The activation token was done. This is used to signal that the window has been activated.
    #[cfg(target_os = "macos")]
    /// The activation token was done. This is used to signal that the window has been activated. It's only available on macOS.
    ActivationTokenDone,
    /// An unknown event occurred.
    Unknown,
}

/// Represents the core device events that can occur in the application.
pub enum CoreDeviceEvent {
    /// A device was connected.
    DeviceConnected,
    /// A device was disconnected.
    DeviceDisconnected,
    /// The mouse was moved. Takes the delta x and y as parameters.
    MouseMotion(f64, f64),
    /// The mouse wheel was scrolled. Takes the delta x and y as parameters.
    MouseWheel(f64, f64),
    /// The motion event occurred. Takes the axis id and value as parameters.
    Motion(u32, f64), // axis, value
    /// A button was pressed or released. Takes the button and state as parameters.
    Button(u32, winit::event::ElementState), // button, state
    /// A key was pressed or released. Takes the keyboard input as a parameter.
    Key(winit::event::KeyboardInput), // input
    /// A text input event occurred. Takes the codepoint as a parameter.
    Text(char), // codepoint
    /// An unknown device event occurred.
    Unknown,
}

/// Represents the core events that can occur in the application.
pub enum CoreEvent {
    /// A window event occurred. Takes the window event as a parameter.
    WindowEvent(CoreWindowEvent),
    /// A device event occurred. Takes the device event as a parameter.
    DeviceEvent(CoreDeviceEvent),
    /// A user-defined event occurred.
    UserEvent,
    /// The application was suspended.
    AppSuspended,
    /// The application was resumed.
    AppResumed,
    /// A memory warning was received.
    MemoryWarning,
}

/// Represents a reference to a core event type.
pub enum CoreEventReference {
    /// A reference to a window event.
    WindowEvent,
    /// A reference to a device event.
    DeviceEvent,
    /// A reference to a user-defined event.
    UserEvent,
    /// A reference to an application suspended event.
    AppSuspended,
    /// A reference to an application resumed event.
    AppResumed,
    /// A reference to a memory warning event.
    MemoryWarning,
}

type DelegateResponse = dyn Fn(&mut winit::window::Window, &mut CoreEvent) + 'static;
struct EventDelegate {
    response: Box<DelegateResponse>,
    event: CoreEventReference,
}

type RenderFunction = dyn Fn(&mut winit::window::Window, &mut crate::macos::metal::MetalRenderer, &mut SharedObjects)
    + 'static;

/// Represents the options for creating a window.
pub struct WindowOptions {
    decorations: bool,
    resizable: bool,
    transparent: bool,
    fullscreen: bool,
    no_titlebar: bool,
}

impl Clone for WindowOptions {
    fn clone(&self) -> Self {
        WindowOptions {
            decorations: self.decorations,
            resizable: self.resizable,
            transparent: self.transparent,
            fullscreen: self.fullscreen,
            no_titlebar: self.no_titlebar,
        }
    }
}

impl Default for WindowOptions {
    fn default() -> Self {
        WindowOptions {
            decorations: true,
            resizable: true,
            transparent: false,
            fullscreen: false,
            no_titlebar: false,
        }
    }
}

impl WindowOptions {
    /// Creates a new instance of `WindowOptions` with no decorations.
    pub fn no_decorations() -> Self {
        WindowOptions {
            decorations: false,
            resizable: true,
            transparent: false,
            fullscreen: false,
            no_titlebar: false,
        }
    }

    /// Creates a new instance of `WindowOptions` with resizable enabled.
    pub fn resizable() -> Self {
        WindowOptions {
            decorations: true,
            resizable: true,
            transparent: false,
            fullscreen: false,
            no_titlebar: false,
        }
    }

    /// Creates a new instance of `WindowOptions` with transparent background.
    pub fn transparent() -> Self {
        WindowOptions {
            decorations: true,
            resizable: true,
            transparent: true,
            fullscreen: false,
            no_titlebar: false,
        }
    }

    /// Creates a new instance of `WindowOptions` with fullscreen enabled.
    pub fn fullscreen() -> Self {
        WindowOptions {
            decorations: true,
            resizable: true,
            transparent: false,
            fullscreen: true,
            no_titlebar: false,
        }
    }

    /// Creates a new instance of `WindowOptions` with no titlebar.
    pub fn no_titlebar() -> Self {
        WindowOptions {
            decorations: true,
            resizable: true,
            transparent: false,
            fullscreen: false,
            no_titlebar: true,
        }
    }
}

/// Represents a window in the application.
pub struct Window {
    /// The title of the window.
    pub title: String,
    /// The width of the window.
    pub width: u32,
    /// The height of the window.
    pub height: u32,
    #[cfg(target_os = "macos")]
    /// The update function that will be called each frame.
    pub update: Box<RenderFunction>,
    /// The background color of the window.
    pub background_color: Color,
    shared_objects: SharedObjects,
    renderer: Box<dyn crate::renderer::Renderer>,
    window: winit::window::Window,
    event_loop: EventLoop<()>,
    events: Vec<EventDelegate>,
}

fn apply_window_options(
    window: &winit::window::WindowBuilder,
    options: &WindowOptions,
) -> winit::window::WindowBuilder {
    let mut builder = window.clone();
    if !options.decorations {
        builder = builder.with_decorations(false);
    }
    if !options.resizable {
        builder = builder.with_resizable(false);
    }
    if options.transparent {
        builder = builder.with_transparent(true);
    }
    if options.fullscreen {
        builder = builder.with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
    }
    builder
}

impl Window {
    /// Creates a new instance of `Window` with the specified title, width, height, and options.
    pub fn new(title: &str, width: u32, height: u32, options: Option<WindowOptions>) -> Self {
        let event_loop = EventLoop::new();
        let mut window_builder = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height));
        let cloned_options = options.clone();
        window_builder = apply_window_options(&window_builder, &options.unwrap_or_default());

        let window = window_builder
            .build(&event_loop)
            .expect("Cannot create window");

        if cloned_options.is_some() && cloned_options.unwrap().no_titlebar {
            #[cfg(target_os = "macos")]
            crate::macos::win_custom::customize_window(&window);
        }
        Window {
            title: title.to_string(),
            width,
            height,
            renderer: create_renderer(&window, Color::new(0.05, 0.05, 0.05, 1.0)),
            window,
            event_loop,
            update: Box::new(|_, _, _| ()),
            shared_objects: SharedObjects::new(),
            events: Vec::new(),
            background_color: Color::new(0.05, 0.05, 0.05, 1.0),
        }
    }

    /// Sets the background color of the window.
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
        self.renderer.set_background_color(color);
    }

    fn execute_event_static(
        window: &mut winit::window::Window,
        events: &[EventDelegate],
        event: &winit::event::Event<()>,
    ) {
        match event {
            Event::WindowEvent { event, .. } => {
                for delegate in events {
                    if let CoreEventReference::WindowEvent = delegate.event {
                        (delegate.response)(
                            window,
                            &mut CoreEvent::WindowEvent(window_from_winit_event(event)),
                        );
                    }
                }
            }
            Event::DeviceEvent { event, .. } => {
                for delegate in events {
                    if let CoreEventReference::DeviceEvent = delegate.event {
                        (delegate.response)(
                            window,
                            &mut CoreEvent::DeviceEvent(device_from_winit_event(event)),
                        );
                    }
                }
            }
            _ => (),
        }
    }

    /// Handles core events and delegates them to the appropriate handlers.
    pub fn handle_core_event(&mut self, core_event: &mut CoreEvent) {
        match core_event {
            CoreEvent::WindowEvent(_) => {
                for delegate in &self.events {
                    if let CoreEventReference::WindowEvent = delegate.event {
                        (delegate.response)(&mut self.window, core_event);
                    }
                }
            }
            CoreEvent::DeviceEvent(_) => {
                for delegate in &self.events {
                    if let CoreEventReference::DeviceEvent = delegate.event {
                        (delegate.response)(&mut self.window, core_event);
                    }
                }
            }
            CoreEvent::UserEvent => {
                for delegate in &self.events {
                    if let CoreEventReference::UserEvent = delegate.event {
                        (delegate.response)(&mut self.window, core_event);
                    }
                }
            }
            CoreEvent::AppSuspended => {
                for delegate in &self.events {
                    if let CoreEventReference::AppSuspended = delegate.event {
                        (delegate.response)(&mut self.window, core_event);
                    }
                }
            }
            CoreEvent::AppResumed => {
                for delegate in &self.events {
                    if let CoreEventReference::AppResumed = delegate.event {
                        (delegate.response)(&mut self.window, core_event);
                    }
                }
            }
            CoreEvent::MemoryWarning => {
                for delegate in &self.events {
                    if let CoreEventReference::MemoryWarning = delegate.event {
                        (delegate.response)(&mut self.window, core_event);
                    }
                }
            }
        }
    }

    /// Adds an event handler for a specific core event type.
    pub fn on_event<F>(&mut self, event_type: CoreEventReference, handler: F)
    where
        F: Fn(&mut winit::window::Window, &mut CoreEvent) + 'static,
    {
        self.events.push(EventDelegate {
            response: Box::new(handler),
            event: event_type,
        });
    }

    #[cfg(target_os = "macos")]
    /// Sets the update function that will be called each frame.
    pub fn each_frame<F>(&mut self, update: F)
    where
        F: Fn(
                &mut winit::window::Window,
                &mut crate::macos::metal::MetalRenderer,
                &mut SharedObjects,
            ) + 'static,
    {
        self.update = Box::new(update);
    }

    /// Shares an object with the window's shared objects.
    pub fn share_object<T: 'static + Send + Sync>(&mut self, object: T) {
        self.shared_objects.add_object(object);
    }

    /// Launches the window and starts the event loop.
    pub fn launch(self) {
        let mut window = self.window;
        let mut objects = self.shared_objects;
        let update = self.update;
        let mut renderer = self.renderer;
        let events = self.events;

        renderer.resize(self.width as f64, self.height as f64);

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            // Handle custom events before processing winit events
            Self::execute_event_static(&mut window, &events, &event);

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        renderer.destroy();
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::Resized(physical_size) => {
                        renderer.resize(physical_size.width as f64, physical_size.height as f64);
                    }
                    _ => (),
                },
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    #[cfg(target_os = "macos")]
                    {
                        if let Some(metal_renderer) = renderer
                            .as_any_mut()
                            .downcast_mut::<crate::macos::metal::MetalRenderer>(
                        ) {
                            (update)(&mut window, metal_renderer, &mut objects);
                        } else {
                            panic!("Renderer is not a MetalRenderer");
                        }
                    }
                    renderer.render(&window);
                }
                _ => (),
            }
        });
    }

    /// Adds an object to the renderer.
    pub fn add_object(&mut self, object: crate::object::Object) {
        self.renderer.add_object(object);
    }

    /// Clears all objects from the renderer.
    pub fn clear(&mut self) {
        self.renderer.clear();
    }

    /// Gets a reference to the shared objects.
    pub fn destroy(&self) {
        self.renderer.destroy();
    }

    /// Sets the title of the window.
    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
        self.window.set_title(&self.title);
    }

    /// Sets the size of the window.
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.window
            .set_inner_size(winit::dpi::LogicalSize::new(width, height));
        self.renderer.resize(width as f64, height as f64);
    }

    /// Gets the current framebuffer size of the window. In macOS and iOS, this is usually twice the size of the window.
    pub fn framebuffer_size(&self) -> [f32; 2] {
        let size = self.window.inner_size();
        [size.width as f32, size.height as f32]
    }
}

fn device_id_to_u32(device_id: &DeviceId) -> u32 {
    let mut hasher = DefaultHasher::new();
    device_id.hash(&mut hasher);
    hasher.finish() as u32
}

fn window_from_winit_event(event: &winit::event::WindowEvent) -> CoreWindowEvent {
    match event {
        WindowEvent::AxisMotion {
            device_id,
            axis,
            value,
        } => CoreWindowEvent::AxisMotion(device_id_to_u32(device_id), *axis, *value),
        WindowEvent::CursorMoved {
            device_id,
            position,
            ..
        } => CoreWindowEvent::CursorMoved(device_id_to_u32(device_id), position.x, position.y),
        WindowEvent::CursorEntered { device_id } => {
            CoreWindowEvent::CursorEntered(device_id_to_u32(device_id))
        }
        WindowEvent::CursorLeft { device_id } => {
            CoreWindowEvent::CursorLeft(device_id_to_u32(device_id))
        }
        WindowEvent::MouseInput {
            device_id,
            state,
            button,
            ..
        } => CoreWindowEvent::MouseClick(device_id_to_u32(device_id), *button, *state),
        WindowEvent::MouseWheel {
            device_id,
            delta,
            phase,
            ..
        } => CoreWindowEvent::MouseScroll(device_id_to_u32(device_id), *delta, *phase),
        WindowEvent::TouchpadPressure {
            device_id,
            pressure,
            ..
        } => CoreWindowEvent::TouchpadPressure(device_id_to_u32(device_id), *pressure as f64),
        WindowEvent::Resized(size) => {
            CoreWindowEvent::Resized(size.width as f64, size.height as f64)
        }
        WindowEvent::Moved(position) => {
            CoreWindowEvent::Moved(position.x as f64, position.y as f64)
        }
        WindowEvent::CloseRequested => CoreWindowEvent::Closing,
        WindowEvent::Destroyed => CoreWindowEvent::Destroyed,
        WindowEvent::DroppedFile(path) => {
            CoreWindowEvent::DroppedFile(path.to_string_lossy().to_string())
        }
        WindowEvent::HoveredFile(path) => {
            CoreWindowEvent::HoveredFile(path.to_string_lossy().to_string())
        }
        WindowEvent::HoveredFileCancelled => CoreWindowEvent::HoveredFileCancelled,
        WindowEvent::Focused(focused) => CoreWindowEvent::Focused(*focused),
        WindowEvent::ReceivedCharacter(character) => CoreWindowEvent::RecievedChar(*character),
        WindowEvent::KeyboardInput { input, .. } => CoreWindowEvent::KeyboardInput(*input),
        WindowEvent::ModifiersChanged(modifiers) => CoreWindowEvent::ModifierChanged(*modifiers),
        WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
            CoreWindowEvent::DPIChanged(*scale_factor, *scale_factor)
        }
        WindowEvent::ThemeChanged(theme) => CoreWindowEvent::ThemeChanged(*theme),
        WindowEvent::Occluded(occluded) => CoreWindowEvent::Occluded(*occluded),

        _ => CoreWindowEvent::Unknown,
    }
}

fn device_from_winit_event(event: &winit::event::DeviceEvent) -> CoreDeviceEvent {
    match event {
        winit::event::DeviceEvent::MouseMotion { delta } => {
            CoreDeviceEvent::MouseMotion(delta.0, delta.1)
        }
        winit::event::DeviceEvent::MouseWheel { delta } => match delta {
            MouseScrollDelta::LineDelta(x, y) => {
                CoreDeviceEvent::MouseWheel(x.abs() as f64, y.abs() as f64)
            }
            MouseScrollDelta::PixelDelta(PhysicalPosition { x, y }) => {
                CoreDeviceEvent::MouseWheel(x.abs(), y.abs())
            }
        },
        winit::event::DeviceEvent::Motion { axis, value } => CoreDeviceEvent::Motion(*axis, *value),
        winit::event::DeviceEvent::Button { button, state } => {
            CoreDeviceEvent::Button(*button, *state)
        }
        winit::event::DeviceEvent::Key(input) => CoreDeviceEvent::Key(*input),
        winit::event::DeviceEvent::Text { codepoint } => CoreDeviceEvent::Text(*codepoint),

        _ => CoreDeviceEvent::Unknown,
    }
}
