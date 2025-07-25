#[cfg(target_os = "macos")]
pub struct Buffer<T> {
    pub data: Vec<T>,
    pub buffer: metal::Buffer,
}

#[cfg(target_os = "macos")]
impl<T> Buffer<T> {
    pub fn new(data: Vec<T>) -> Self {
        let device = metal::Device::system_default().expect("No Metal device found");
        let buffer = device.new_buffer_with_data(
            data.as_ptr() as *const std::ffi::c_void,
            (data.len() * std::mem::size_of::<T>()) as u64,
            metal::MTLResourceOptions::CPUCacheModeDefaultCache,
        );
        Buffer { data, buffer }
    }

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
