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
    //let jump = { move |route| Callback::from(move |_| clone_navigator.push(&route)) };

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
            <nav class="bg-gray-800">
                



                    <div class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
                      <div class="relative flex h-16 items-center justify-between">
                        <div class="absolute inset-y-0 left-0 flex items-center sm:hidden">
                          <button type="button" class="inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white" aria-controls="mobile-menu" aria-expanded="false">
                            <span class="sr-only">{"Open main menu"}</span>
                            <svg class="block h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                              <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                            </svg>
                            <svg class="hidden h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                              <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                          </button>
                        </div>
                        <div class="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
                          <div class="flex flex-shrink-0 items-center">
                            <img class="block h-8 w-auto lg:hidden" src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=500" alt="Your Company"/>
                            <img class="hidden h-8 w-auto lg:block" src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=500" alt="Your Company"/>
                          </div>
                          <div class="hidden sm:ml-6 sm:block">
                            <div class="flex space-x-4">
                              <a href="#" class="bg-gray-900 text-white rounded-md px-3 py-2 text-sm font-medium" aria-current="page">{"Dashboard"}</a>
                              <a href="#" class="text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">{"Team"}</a>
                              <a href="#" class="text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">{"Projects"}</a>
                              <a href="#" class="text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">{"Calendar"}</a>
                            </div>
                          </div>
                        </div>
                        <div class="absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0">
                          <button type="button" class="rounded-full bg-gray-800 p-1 text-gray-400 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800">
                            <span class="sr-only">{"View notifications"}</span>
                            <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                              <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0" />
                            </svg>
                          </button>
                  
                          <div class="relative ml-3">
                            <div>
                              <button type="button" class="flex rounded-full bg-gray-800 text-sm focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800" id="user-menu-button" aria-expanded="false" aria-haspopup="true">
                                <span class="sr-only">{"Open user menu"}</span>
                                <img class="h-8 w-8 rounded-full" src={user.avatar_url} title={format!("Hi, {}!", user.login)} alt=""/>
                              </button>
                            </div>
                  
                            <div class="absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none" role="menu" aria-orientation="vertical" aria-labelledby="user-menu-button" tabindex="-1">
                              <a href="#" class="block px-4 py-2 text-sm text-gray-700" role="menuitem" tabindex="-1" id="user-menu-item-0">{"Your Profile"}</a>
                              <a href="#" class="block px-4 py-2 text-sm text-gray-700" role="menuitem" tabindex="-1" id="user-menu-item-1">{"Settings"}</a>
                              <a href="#" class="block px-4 py-2 text-sm text-gray-700" role="menuitem" tabindex="-1" id="user-menu-item-2">{"Sign out"}</a>
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  
                    <div class="sm:hidden" id="mobile-menu">
                      <div class="space-y-1 px-2 pb-3 pt-2">
                        <a href="#" class="bg-gray-900 text-white block rounded-md px-3 py-2 text-base font-medium" aria-current="page">{"Dashboard"}</a>
                        <a href="#" class="text-gray-300 hover:bg-gray-700 hover:text-white block rounded-md px-3 py-2 text-base font-medium">{"Team"}</a>
                        <a href="#" class="text-gray-300 hover:bg-gray-700 hover:text-white block rounded-md px-3 py-2 text-base font-medium">{"Projects"}</a>
                        <a href="#" class="text-gray-300 hover:bg-gray-700 hover:text-white block rounded-md px-3 py-2 text-base font-medium">{"Calendar"}</a>
                      </div>
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
