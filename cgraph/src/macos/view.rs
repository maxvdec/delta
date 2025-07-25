#![allow(unexpected_cfgs)]

use cocoa::{
    base::{YES, id, nil},
    foundation::NSAutoreleasePool,
};
use metal::{Device, DeviceRef, MetalLayer};

use objc::{msg_send, *};
use winit::{platform::macos::WindowExtMacOS, window::Window};

pub fn setup_layer(device: &DeviceRef, window: &Window) -> MetalLayer {
    let ns_view = window.ns_view() as id;

    let layer = MetalLayer::new();
    layer.set_device(device);
    layer.set_pixel_format(metal::MTLPixelFormat::BGRA8Unorm);
    layer.set_presents_with_transaction(false);

    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        let () = msg_send![ns_view, setLayer: layer.as_ref()];
        let () = msg_send![ns_view, setWantsLayer: YES];
    }

    layer
}
