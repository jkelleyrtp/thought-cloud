#[derive(Clone, Debug)]
pub struct ThoughtItem {
    pub value: String,
}

#[derive(Clone, Debug)]
pub struct StoreModel {
    pub items: Vec<ThoughtItem>,
    pub document_title: String,
}

impl PartialEq for StoreModel {
    fn eq(&self, other: &StoreModel) -> bool {
        false
    }
}
