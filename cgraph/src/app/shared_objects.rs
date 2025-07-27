pub struct SharedObjects {
    pub objects: Vec<Box<dyn std::any::Any>>,
}

// Macro to get a single object by index
#[macro_export]
macro_rules! get_object {
    ($objects:expr, $type:ty, $index:expr) => {
        $objects.get_object_at_mut::<$type>($index)
    };
}

// Macro to get all objects of a type
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
    pub fn new() -> Self {
        SharedObjects {
            objects: Vec::new(),
        }
    }

    pub fn add_object<T: 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }

    pub fn get_object<T: 'static>(&self) -> Vec<&T> {
        self.objects
            .iter()
            .filter_map(|obj| obj.downcast_ref::<T>())
            .collect()
    }

    pub fn get_object_mut<T: 'static>(&mut self) -> Vec<&mut T> {
        self.objects
            .iter_mut()
            .filter_map(|obj| obj.downcast_mut::<T>())
            .collect()
    }

    pub fn get_object_at<T: 'static>(&self, index: usize) -> Option<&T> {
        if index < self.objects.len() {
            self.objects[index].downcast_ref::<T>()
        } else {
            None
        }
    }

    pub fn get_object_at_mut<T: 'static>(&mut self, index: usize) -> Option<&mut T> {
        if index < self.objects.len() {
            self.objects[index].downcast_mut::<T>()
        } else {
            None
        }
    }
}
