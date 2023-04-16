use yew::prelude::*;

#[function_component(Dog)]
pub fn artist_card() -> Html {
    html! {
        <div class="flex flex-col justify-center items-center gap-y-6 min-h-screen bg-base-200">
            <script src="img/lottie.js" />
            <lottie-player src="img/dog.json" background="transparent" speed="1" style="width: 300px; height: 300px;"
                loop={true} autoplay={true} />
            <h1 class="text-4xl font-bold font-display">{"Retrieving your music :)"}</h1>
        </div>
    }
}