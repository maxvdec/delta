#[cfg(target_os = "macos")]
#[derive(Debug)]
/// Represents a buffer for storing vertex data in Metal.
pub struct Buffer<T> {
    /// The data stored in the buffer.
    pub data: Vec<T>,
    /// The Metal buffer object.
    pub buffer: metal::Buffer,
}

#[cfg(target_os = "macos")]
impl<T> Buffer<T> {
    /// Creates a new buffer with the given data.
    pub fn new(data: Vec<T>) -> Self {
        let device = metal::Device::system_default().expect("No Metal device found");
        let buffer = device.new_buffer_with_data(
            data.as_ptr() as *const std::ffi::c_void,
            (data.len() * std::mem::size_of::<T>()) as u64,
            metal::MTLResourceOptions::CPUCacheModeDefaultCache,
        );
        Buffer { data, buffer }
    }

    /// Updates the buffer with new data.
    pub fn update(&mut self, data: Vec<T>) {
        self.data = data;
        let device = metal::Device::system_default().expect("No Metal device found");
        self.buffer = device.new_buffer_with_data(
            self.data.as_ptr() as *const std::ffi::c_void,
            (self.data.len() * std::mem::size_of::<T>()) as u64,
            metal::MTLResourceOptions::CPUCacheModeDefaultCache,
        );
    }
}

impl<T: Clone> Clone for Buffer<T> {
    fn clone(&self) -> Self {
        Buffer {
            data: self.data.clone(),
            buffer: self.buffer.clone(),
        }
    }
}
