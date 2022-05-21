use yew::prelude::*;
use serde_json::Value;
use web_sys;

#[derive(Clone, Debug, PartialEq)]
struct Weather {
    temperature: i32,
    unit: String,
}

async fn fetch_weather_data() ->  Weather {
    const WEATHER_API_URL: &str = "https://api.weather.gov/gridpoints/TOP/31,80/forecast";

    let response = reqwest::get(WEATHER_API_URL).await.unwrap();

    let text = response.text().await.unwrap();

    let serialized_response: Value = serde_json::from_str(&text).unwrap();

    let temperature = serialized_response["properties"]["periods"][0]["temperature"].to_string();
    let unit = serialized_response["properties"]["periods"][0]["temperatureUnit"].as_str().unwrap().to_string();

    return Weather {
        temperature: temperature.parse::<i32>().unwrap(),
        unit,
    };
}

#[function_component(App)]
fn app() -> Html {

    let weather_state = use_state_eq::<Option<Weather>, _>(|| None);
    let weather_state_outer = weather_state.clone();

    wasm_bindgen_futures::spawn_local(async move {
        let weather = fetch_weather_data().await;
        web_sys::console::log_1(&format!("{}", weather.unit).into());
        let weather_state = weather_state.clone();
        weather_state.set(Some(weather));
    });

    match (*weather_state_outer).clone() {
        Some(weather) => {
            html! {
                <div>
                    <h1>{"Weather Application"}</h1>
                    <h3>{"The current weather for Washington, USA is:"}</h3>
                    <p>{ format!("{}° {}", weather.temperature, weather.unit.to_string()) }</p>
                </div>
            }
        },
        None => {
            html! {
                <div>
                    <h3>{"Loading..."}</h3>
                </div>
            }
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
