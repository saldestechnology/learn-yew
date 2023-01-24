use markdown;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };
    let md = markdown::to_html("# Hello, world!");
    html! {
        <main>
            {Html::from_html_unchecked(md.into())}
            <button {onclick}>{"+1"}</button>
            <p>{*counter}</p>
        </main>
    }
}
