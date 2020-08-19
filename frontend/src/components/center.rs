use crate::state::selector::current_thoughtitems;
use crate::{state::StoreModel, util::handle_future};
use closure::closure;
use fc_macros::{callback_once, functional_component};
use reqwest;
use std::rc::Rc;
use yew::{html, Callback, Html};
use yew_functional::{use_context, use_state, FunctionComponent, FunctionProvider};

#[functional_component]
pub fn center() -> Html {
    let ctx = use_context::<Rc<StoreModel>>();
    let thing = current_thoughtitems(ctx.as_ref());

    let things = thing
        .iter()
        .map(|f| html! {<li>{&f.value}</li>})
        .collect::<Vec<Html>>();

    html! {
        <div class="Center">
            <ul>
                {things}
            </ul>
        </div>
    }
}
