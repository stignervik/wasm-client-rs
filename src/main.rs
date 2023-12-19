use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = leptos::create_signal(0);

    let double_count = move || count() * 2;

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1)
                //set_count(3)
            }

            class:red=move || count() % 2 == 1
            class:green=move || count() % 2 == 0
            class=("button-20", move || count() % 2 == 1)
            // class=("name", blue)=move || count() % 2 = 0
        >
            "Click me count: "
            // {move || count.get()}
            {count}
        </button>
        <br></br>
        <ProgressBar progress=count max=10/>
        <br></br>

        <ProgressBar progress=double_count/>
        <p>
        "Double Count: "
        // and again here
        {double_count}
        </p>
    }
}

#[component]
fn ProgressBar<F: Fn() -> i32 + 'static>(
    #[prop(default = 100)] max: u16,
    progress: F,
) -> impl IntoView {
    view! {
        <progress
            max=max
            // now this works
            value=progress
        />
    }
}
