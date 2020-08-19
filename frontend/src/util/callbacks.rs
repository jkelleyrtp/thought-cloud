use std::rc::Rc;
use yew::Callback;

pub fn CB<T, F: Fn(T) + 'static>(infn: F) -> Callback<T> {
    let newfn = Rc::new(infn);
    Callback::Callback(newfn)
}
