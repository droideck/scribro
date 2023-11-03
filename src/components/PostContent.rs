use leptos::*;

#[server(PerformMarkdownCodeToHtml, "/api", "GetJSON")]
pub async fn perform_markdown_code_to_html(markdown: String) -> Result<String, ServerFnError> {
    use cached::proc_macro::cached;

    #[cached]
    fn process_markdown(markdown: String) -> Result<String, ServerFnError> {
        use femark::{process_markdown_to_html, HTMLOutput};

        match process_markdown_to_html(&markdown) {
            Ok(HTMLOutput { content, toc: _, .. }) => Ok(content),
            Err(e) => Err(ServerFnError::ServerError(e.to_string())),
        }
    }

    process_markdown(markdown)
}


#[component]
pub fn PostContent(
    text: String,
) -> impl IntoView {
    let text_resource = create_resource(
        || false,
        move |_| perform_markdown_code_to_html(text.clone()),
    );

    view! {
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
    code: TextMode,
) -> impl IntoView {

    let _content = {match code {
        TextMode::Html(text_resource) => {
            view! {
                <Suspense fallback=move || {
                    view! { <div>"fallback"</div> }
                }>
                    {move || {
                        text_resource
                            .get()
                            .map(|res| {
                                res.map(|text| {
                                    view! { <article class="prose lg:prose-xl" inner_html=text /> }
                                })
                            })
                    }}
                </Suspense>
            }
                .into_view()
        }
        TextMode::View(child) => {
            view! { <div>
                <p>"Nope"</p>{child}</div> }
                .into_view()
        }
    }};

    view! { _content
    }
}