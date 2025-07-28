use std::hash::{DefaultHasher, Hash, Hasher};

use winit::{
    dpi::PhysicalPosition,
    event::{DeviceId, Event, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{app::SharedObjects, renderer::create_renderer};
#[cfg(target_os = "macos")]
pub struct Context {
    pub name: String,
    pub version: String,
    pub description: String,
}

pub enum CoreWindowEvent {
    Resized(f64, f64), // x, y
    Moved(f64, f64),   // x, y
    Closing,
    Destroyed,
    DroppedFile(String), // path
    HoveredFile(String), // path
    HoveredFileCancelled,
    Focused(bool),                                 // focused
    RecievedChar(char),                            // character
    KeyboardInput(winit::event::KeyboardInput),    // input
    ModifierChanged(winit::event::ModifiersState), // modifiers
    CursorMoved(u32, f64, f64),                    // device id, x, y
    CursorEntered(u32),                            // device id
    CursorLeft(u32),                               // device id
    MouseScroll(
        u32,
        winit::event::MouseScrollDelta,
        winit::event::TouchPhase,
    ), // device id, delta, phase
    MouseClick(u32, winit::event::MouseButton, winit::event::ElementState), // device id, button, state
    TouchpadPressure(u32, f64),                                             // device id, pressure
    AxisMotion(u32, winit::event::AxisId, f64), // device id, axis id, value
    Touch(winit::event::Touch),                 // touch event
    DPIChanged(f64, f64),                       // scale x, scale y
    ThemeChanged(winit::window::Theme),         // theme
    Occluded(bool),                             // occluded
    #[cfg(target_os = "macos")]
    ActivationTokenDone,
    Unknown,
}

pub enum CoreDeviceEvent {
    DeviceConnected,
    DeviceDisconnected,
    MouseMotion(f64, f64),
    MouseWheel(f64, f64),
    Motion(u32, f64),                        // axis, value
    Button(u32, winit::event::ElementState), // button, state
    Key(winit::event::KeyboardInput),        // input
    Text(char),                              // codepoint
    Unknown,
}

pub enum CoreEvent {
    WindowEvent(CoreWindowEvent),
    DeviceEvent(CoreDeviceEvent),
    UserEvent,
    AppSuspended,
    AppResumed,
    MemoryWarning,
}

pub enum CoreEventReference {
    WindowEvent,
    DeviceEvent,
    UserEvent,
    AppSuspended,
    AppResumed,
    MemoryWarning,
}

type DelegateResponse = dyn Fn(&mut winit::window::Window, &mut CoreEvent) + 'static;
struct EventDelegate {
    response: Box<DelegateResponse>,
    event: CoreEventReference,
}

type RenderFunction = dyn Fn(&mut winit::window::Window, &mut crate::macos::metal::MetalRenderer, &mut SharedObjects)
    + 'static;

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
    pub fn no_decorations() -> Self {
        WindowOptions {
            decorations: false,
            resizable: true,
            transparent: false,
            fullscreen: false,
            no_titlebar: false,
        }
    }

    pub fn resizable() -> Self {
        WindowOptions {
            decorations: true,
            resizable: true,
            transparent: false,
            fullscreen: false,
            no_titlebar: false,
        }
    }

    pub fn transparent() -> Self {
        WindowOptions {
            decorations: true,
            resizable: true,
            transparent: true,
            fullscreen: false,
            no_titlebar: false,
        }
    }

    pub fn fullscreen() -> Self {
        WindowOptions {
            decorations: true,
            resizable: true,
            transparent: false,
            fullscreen: true,
            no_titlebar: false,
        }
    }

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

pub struct Window {
    pub title: String,
    pub width: u32,
    pub height: u32,
    #[cfg(target_os = "macos")]
    pub update: Box<RenderFunction>,
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
    pub fn new(title: &str, width: u32, height: u32, options: Option<WindowOptions>) -> Self {
        let event_loop = EventLoop::new();
        let mut window_builder = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height));
        let cloned_options = options.clone();
        window_builder = apply_window_options(
            &window_builder,
            &options.unwrap_or(WindowOptions::default()),
        );

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
            renderer: create_renderer(&window),
            window,
            event_loop,
            update: Box::new(|_, _, _| ()),
            shared_objects: SharedObjects::new(),
            events: Vec::new(),
        }
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

    pub fn share_object<T: 'static + Send + Sync>(&mut self, object: T) {
        self.shared_objects.add_object(object);
    }

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

    pub fn add_object(&mut self, object: crate::object::Object) {
        self.renderer.add_object(object);
    }

    pub fn clear(&mut self) {
        self.renderer.clear();
    }

    pub fn destroy(&self) {
        self.renderer.destroy();
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
