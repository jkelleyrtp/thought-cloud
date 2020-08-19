pub mod reducer;
pub mod selector;
pub mod slice;
pub use slice::StoreModel;
use std::rc::Rc;
use yew_functional::{use_context, ContextProvider};

pub type StoreModelContextProvider = ContextProvider<Rc<StoreModel>>;
pub type StoreDispatchContextProvider = ContextProvider<StoreDispatch>;

pub use reducer::{reducer, Action, StoreDispatch};
