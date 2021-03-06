use crate::util::handle_future;
use closure::closure;
use fc_macros::{callback_once, functional_component};
use reqwest;
use yew::{html, Callback, Html};
use yew_functional::{use_state, FunctionComponent, FunctionProvider};

#[functional_component]
pub fn left_side_bar() -> Html {
    let (content, set_content) = use_state(|| vec![0.0]);

    html! {
        <div class="LeftSidebar">
            {"LeftSidebar"}
        </div>
    }
}
