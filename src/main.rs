use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"Weather Application"}</h1>
            <h3>{"The current weather for Toronto is:"}</h3>
            <p>{"It is currently sunny"}</p>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
