use crate::hooks::UseEthereumHandle;
use yew::prelude::*;

#[function_component]
pub fn AccountLabel() -> Html {
    let ethereum = use_context::<Option<UseEthereumHandle>>().expect(
        "no ethereum provider found. you must wrap your components in an <EthereumProvider/>",
    );

    html! {
        <div>
            if let Some(ethereum) = ethereum {
                if ethereum.connected() {
                    {ethereum.display_address()}
                } else {
                    {"Disconnected"}
                }
            } else {
                {"No ethereum provider found"}
            }
        </div>
    }
}
