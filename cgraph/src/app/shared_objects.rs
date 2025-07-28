/// Strcuture that encapsulates shared objects in the application.
pub struct SharedObjects {
    /// A vector that holds all shared objects.
    pub objects: Vec<Box<dyn std::any::Any>>,
}

/// Macro that gets a single object by index from the shared objects.
#[macro_export]
macro_rules! get_object {
    ($objects:expr, $type:ty, $index:expr) => {
        $objects.get_object_at_mut::<$type>($index)
    };
}

/// Macro that gets all objects of a type from the shared objects.
#[macro_export]
macro_rules! get_objects {
    ($objects:expr, $type:ty) => {
        $objects.get_object_mut::<$type>()
    };
}

impl Default for SharedObjects {
    fn default() -> Self {
        SharedObjects::new()
    }
}

impl SharedObjects {
    /// Creates a new instance of `SharedObjects`.
    pub fn new() -> Self {
        SharedObjects {
            objects: Vec::new(),
        }
    }

    /// Adds a new object to the shared objects.
    pub fn add_object<T: 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }

    /// Removes an object from the shared objects by index.
    pub fn get_object<T: 'static>(&self) -> Vec<&T> {
        self.objects
            .iter()
            .filter_map(|obj| obj.downcast_ref::<T>())
            .collect()
    }

    /// Gets a mutable reference to all objects of a type.
    pub fn get_object_mut<T: 'static>(&mut self) -> Vec<&mut T> {
        self.objects
            .iter_mut()
            .filter_map(|obj| obj.downcast_mut::<T>())
            .collect()
    }

    /// Gets a reference to an object at a specific index.
    pub fn get_object_at<T: 'static>(&self, index: usize) -> Option<&T> {
        if index < self.objects.len() {
            self.objects[index].downcast_ref::<T>()
        } else {
            None
        }
    }

    /// Gets a mutable reference to an object at a specific index.
    pub fn get_object_at_mut<T: 'static>(&mut self, index: usize) -> Option<&mut T> {
        if index < self.objects.len() {
            self.objects[index].downcast_mut::<T>()
        } else {
            None
        }
    }
}
