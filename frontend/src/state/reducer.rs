use super::{slice::ThoughtItem, StoreModel};

#[derive(Clone)]
pub struct StoreDispatch(pub std::rc::Rc<dyn std::ops::Fn(Action) -> ()>);

impl PartialEq for StoreDispatch {
    fn eq(&self, other: &StoreDispatch) -> bool {
        false
    }
}

impl StoreDispatch {
    pub fn emit(&self, action: Action) -> () {
        (self.0)(action);
    }
}

pub enum Action {
    AddThoughtItem(ThoughtItem),
}

pub fn reducer(prev: std::rc::Rc<StoreModel>, action: Action) -> StoreModel {
    let StoreModel {
        items,
        document_title,
    } = &*prev;

    match action {
        Action::AddThoughtItem(item) => StoreModel {
            items: [items.clone(), vec![item]].concat(),
            document_title: document_title.clone(),
            ..*prev
        },
    }
}
