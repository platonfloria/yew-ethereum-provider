use crate::{
    hooks::UseEthereumHandle,
    Chain,
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub chain: Chain,

    #[prop_or_default]
    pub class: Option<String>,
}

#[function_component]
pub fn SwitchNetworkButton(props: &Props) -> Html {
    let ethereum = use_context::<Option<UseEthereumHandle>>().expect(
        "no ethereum ethereum found. you must wrap your components in an <Ethereumethereum/>",
    );

    if let Some(ethereum) = ethereum {
        let chain = props.chain.clone();

        let on_click = {
            let ethereum = ethereum.clone();
            Callback::from(move |_| {
                let ethereum = ethereum.clone();
                let chain = chain.clone();
                spawn_local(async move {
                    let _ = ethereum.switch_chain_with_fallback(&chain).await;
                });
            })
        };

        html! {
            <div>
                <button onclick={on_click} class={&props.class}>
                    {"Switch to "}{&props.chain.chain_name}
                </button>
            </div>
        }
    } else {
        html! {}
    }
}
