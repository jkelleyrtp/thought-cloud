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

use std::borrow::Cow;

#[functional_component]
pub fn bottom() -> Html {
    let ctx = use_context::<Rc<StoreModel>>();
    let dispatch = use_context::<StoreDispatch>().unwrap();

    let cb2: Cow<'_, Rc<Rc<StoreModel>>> = Cow::Owned(ctx.unwrap());

    let (searchinput, setsearch_input) = use_state(|| format!(""));
    let setsearch_input = Rc::new(setsearch_input);

    let handle_input = callback!(clone setsearch_input, clone cb2, |e: InputData| {
        setsearch_input(e.value);
        log::info!("hello {:#?}", cb2);
    });

    let handle_enter = callback!(clone setsearch_input, clone dispatch, clone searchinput, clone cb2, |e: KeyboardEvent| {
        log::info!("hello {:#?}", cb2);
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
                <input
                    placeholder="Search..."
                    class="search-bar"
                    />
            </div>
        </div>
    }
}
