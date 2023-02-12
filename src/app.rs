use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>

        // sets the document title
        <Title text="kukkaro"/>

        // content for this welcome page
        <Router>
            <nav>
                // LR will enhance the active <a> link with the [aria-current] attribute
                // we can use this for styling them with CSS like `[aria-current] { font-weight: bold; }`
                <ul>
                    <li><A href="">"kukkaro"</A></li>
                    <li><A href="create_new_account">"Create new account"</A></li>
                    <li><A href="settings">"Settings"</A></li>
                </ul>
            </nav>
            
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="create_new_account" view=|cx| view! { cx, <CreateNewAccount/> }/>
                    <Route path="settings" view=|cx| view! { cx, <Settings/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"kukkaro"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
fn CreateNewAccount(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"new account"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
fn Settings(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"settings"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

