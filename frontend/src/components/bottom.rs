use crate::{
    state::{slice::ThoughtItem, Action, StoreDispatch, StoreModel},
    util::handle_future,
};
use closure::closure;
use fc_macros::{callback, callback_once, functional_component};
use reqwest;
use std::rc::Rc;
use yew::{
    events::{InputData, KeyboardEvent},
    html, Callback, Html,
};
use yew_functional::{use_context, use_state, FunctionComponent, FunctionProvider};

#[functional_component]
pub fn bottom() -> Html {
    let ctx = use_context::<Rc<StoreModel>>();
    let dispatch = use_context::<StoreDispatch>().unwrap();

    let (searchinput, setsearch_input) = use_state(|| format!(""));
    let setsearch_input = Rc::new(setsearch_input);

    let handle_input = callback!(clone setsearch_input, |e: InputData| setsearch_input(e.value));

    let handle_enter = callback!(clone setsearch_input, clone dispatch, clone searchinput, |e: KeyboardEvent| {
        if e.key() == "Enter" {
            dispatch.emit(Action::AddThoughtItem(ThoughtItem {value: searchinput.as_ref().clone()}));
            setsearch_input(format!(""));
        }
    });

    html! {
        <div class="Bottom">
            <div class="searchbar">
                <input
                    placeholder="Insert a new link.."
                    class="search-bar"
                    value=searchinput
                    oninput=handle_input
                    onkeypress=handle_enter
                    />
            </div>
        </div>
    }
}
