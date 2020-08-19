use crate::{
    components::{Bottom, Center, HeaderBar, LeftSideBar, RightSideBar, TopLeft, TopRight},
    state::{
        reducer, StoreDispatch, StoreDispatchContextProvider, StoreModel, StoreModelContextProvider,
    },
};
use fc_macros::functional_component;

use yew::{html, Html};
use yew_functional::{use_reducer_with_init, use_state, FunctionComponent, FunctionProvider};

#[functional_component]
pub fn my_app() -> Html {
    let initial_state = StoreModel { items: vec![] };

    let (store, dispatch) =
        use_reducer_with_init(reducer, initial_state, |initail_state: StoreModel| {
            initail_state
        });

    let dispatch = StoreDispatch(dispatch);

    html! {
        <>
            <StoreDispatchContextProvider context=dispatch>
                <StoreModelContextProvider context=store>
                    <div class="app-container">
                        <div class="graph-container">
                            {"hello"}
                        </div>
                        <div class="grid-container">
                            <Bottom />
                            <LeftSideBar />
                            <RightSideBar />
                            <TopLeft />
                            <TopRight />
                            <Center />
                            <HeaderBar />
                        </div>
                    </div>
                </StoreModelContextProvider >
            </StoreDispatchContextProvider >
        </>
    }
}
