use crate::{
    components::{
        container::Container,
    },
    pages::{
        article::{article_viewer::ArticleViewer, new::NewArticle},
        user::{login::Login, oauth::OAuth},
        not_found::NotFound,
        home::Home,
    },
};

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

/// 主路由
#[derive(Debug, Clone, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/article/:article_id")]
    ArticleViewer { article_id: u32 },
    #[at("/article/new")]
    NewArticle,
    #[at("/user/login")]
    Login,
    #[at("/user/login/oauth")]
    OAuth,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    html! {
        <Container>
        {
            match route {
                Route::Home => html! { <Home/> },
                Route::ArticleViewer { article_id } => html! { <ArticleViewer {article_id}/> },
                Route::NewArticle => html! { <NewArticle/> },
                Route::Login => html! { <Login/> },
                Route::OAuth => html! { <OAuth/> },
                Route::NotFound => html! { <NotFound/> },
            }
        }
        </Container>
    }
}