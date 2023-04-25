use gloo::net::http::Method;
use gloo_console::log;
use yew::prelude::*;
use yew_router::prelude::{use_navigator,use_route};

use crate::{app::Route, api::fetch, models::user::User};

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

/// 应用程序的 Context
#[derive(Debug, Clone, PartialEq)]
pub struct AppContext {
    /// 设置网页的标题
    pub set_title: Callback<String>,
    /// 用户信息（是一个 State，因为我们可能要修改里面的数据，并且修改后要更新显示的数据）
    pub user: UseStateHandle<Result<User, String>>,
}

#[function_component(Container)]
pub fn container(props: &Props) -> Html {
    // 用于跳转到不同的路由
    let navigator = use_navigator().unwrap();

    let local_route:Route = use_route().unwrap_or_default();

    let set_title = Callback::from(move |content: String| {
        // 设置网页的标题
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .set_title(&format!("{content} - Blog"));
    });

    let clone_navigator = navigator.clone();
    // 用于跳转到不同的页面
    let jump = { move |route| Callback::from(move |_| clone_navigator.push(&route)) };

    // 获取用户数据，并放在 Context 里以便使用

    let user = use_state(|| Err("".into()));

    {
        let user = user.clone();
        // 在组件挂载成功时获取用户数据
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetch_user = fetch::fetch::<User>("/apis/users/info".into(), Method::GET, None, None).await;
                    user.set(fetch_user.clone());
                    match &fetch_user {
                      Ok(fu) => log!("get user",fu.clone().login),
                      Err(err) => {log!("not get user",err,local_route != Route::OAuth); 
                                            if local_route != Route::OAuth {  navigator.push(&Route::Login)}},
                    };
                })
            },
            (),
        );
    }

    // 应用程序的 Context
    let context = AppContext { set_title, user };

    html! {
        <>
            if let Ok(user) = (*context.user).clone() {
            <nav>
                <a onclick={
                    // 需要 clone 一下，以便我们下面多次调用这个闭包
                    let jump = jump.clone();
                    jump(Route::Home)
                } class="brand">
                    <span>{ "Blog" }</span>
                </a>
                <div class="menu">
                        <img src={user.avatar_url} title={format!("Hi, {}!", user.login)} style="width: 7%; border-radius: 50%; float: right;"/>
                </div>

            </nav>
            } else {
                // 用户没有登录或者获取用户信息失败
                //<button class="success icon-puzzle" onclick={jump(Route::Login)}>{ "登录" }</button>
            }

            <ContextProvider<AppContext> {context}>
                { for props.children.iter() }
            </ContextProvider<AppContext>>

        </>
    }
}
