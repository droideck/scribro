use leptos::*;

#[server(PerformMarkdownCodeToHtml, "/api", "GetJSON")]
pub async fn perform_markdown_code_to_html(markdown: String) -> Result<String, ServerFnError> {
    use cached::proc_macro::cached;

    #[cached]
    fn process_markdown(markdown: String) -> Result<String, ServerFnError> {
        use femark::{process_markdown_to_html, HTMLOutput};

        match process_markdown_to_html(markdown) {
            Ok(HTMLOutput { content, toc: _ }) => Ok(content),
            Err(e) => Err(ServerFnError::ServerError(e.to_string())),
        }
    }

    process_markdown(markdown)
}


#[component]
pub fn PostContent(
    cx: Scope,
    text: String,
) -> impl IntoView {
    let text_resource = create_resource(
        cx,
        || false,
        move |_| perform_markdown_code_to_html(text.clone()),
    );

    view! { cx,
        <PostContentLayout
            code=TextMode::Html(text_resource)
        />
    }
}

#[derive(Clone)]
pub enum TextMode {
    Html(Resource<bool, Result<String, ServerFnError>>),
    View(View),
}

#[component]
pub fn PostContentLayout(
    cx: Scope,
    code: TextMode,
) -> impl IntoView {

    let content = {match code {
        TextMode::Html(text_resource) => {
            view! { cx,
                <Suspense fallback=move || {
                    view! { cx, <div>"fallback"</div> }
                }>
                    {move || {
                        text_resource
                            .read(cx)
                            .map(|res| {
                                res.map(|text| {
                                    view! { cx, <article class="prose lg:prose-xl" inner_html=text /> }
                                })
                            })
                    }}
                </Suspense>
            }
                .into_view(cx)
        }
        TextMode::View(child) => {
            view! { cx, <div>
                <p>"Nope"</p>{child}</div> }
                .into_view(cx)
        }
    }};

    view! { cx, {content}
    }
}