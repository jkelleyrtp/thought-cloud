use super::reducer::StoreDispatch;
use super::slice::{StoreModel, ThoughtItem};
use std::rc::Rc;
use yew_functional::use_context;

pub type ModelContext<'a> = Option<&'a Rc<Rc<StoreModel>>>;

pub fn current_thoughtitems(context: ModelContext) -> &Vec<ThoughtItem> {
    &context.unwrap().items
}
