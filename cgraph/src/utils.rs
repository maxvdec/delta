#[allow(dead_code)]
pub fn try_downcast<T: 'static>(obj: Box<dyn std::any::Any>) -> Option<Box<T>> {
    if let Ok(obj) = obj.downcast::<T>() {
        Some(obj)
    } else {
        None
    }
}
