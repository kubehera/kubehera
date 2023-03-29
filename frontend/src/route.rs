use yew_router::prelude::*;
use yew::prelude::*;
use crate::components::login::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Login,
    //#[at("/login")]
    //Login,
    #[at("/home")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
pub fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

pub fn switch(routes: Route) -> Html {
    //let user = match use_user() {
    //    Some(user) => user,
        // Redirects to the login page when user is `None`.
    //    None => return html! {
    //        <Redirect<Route> to={Route::Login}/>
    //    },
    //};
    match routes {
        Route::Login => html! { <Login /> },
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}