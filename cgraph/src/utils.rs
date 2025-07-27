#[allow(dead_code)]
pub fn try_downcast<T: 'static>(obj: Box<dyn std::any::Any>) -> Option<Box<T>> {
    obj.downcast::<T>().ok()
}
