use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{models::article::Article, utils};

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or("".into())]
    pub title: AttrValue,
    #[prop_or("".into())]
    pub content: AttrValue,
    pub submit: Callback<Article>,
}

#[function_component(MarkdownEditor)]
pub fn markdown_editor(props: &Props) -> Html {
    let title = use_state(|| props.title.clone());
    let content = use_state(|| props.content.clone());

    let update_title_preview = {
        let title = title.clone();
        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            title.set(value.into())
        })
    };

    let update_content_preview = {
        let content = content.clone();
        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            content.set(value.into())
        })
    };

    let call_submit = {
        let title = title.clone();
        let content = content.clone();
        let submit = props.submit.clone();

        Callback::from(move |_| {
            let article = Article {
                id: None,
                title: (*title).clone().to_string(),
                content: (*content).clone().to_string(),
                date: None,
            };

            submit.emit(article);
        })
    };

    html! {
        <>
        <form style="width: 45%; float: left; margin-left: 2%;">
            <input type="text" placeholder="文章标题" style="margin-bottom: 1%;" oninput={update_title_preview}/>
            <textarea style="margin-bottom: 1%;" placeholder="文章内容（支持使用 Markdown 编辑）" oninput={update_content_preview}>{ &props.content }</textarea>
            <input type="button" value="提交" onclick={call_submit} class="button"/>
        </form>

        <div style="width: 45%; float: right; margin-left: 2%;">
            <article class="card" style="margin: auto; width: 80%;">
                <header>
                    <h3>{ (*title).clone() }</h3>
                </header>
                <footer>
                    { utils::convert_markdown_to_html((*content).to_string()) }
                </footer>
            </article>
        </div>
        </>
    }
}
