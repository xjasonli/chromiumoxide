use futures::StreamExt;

use chromiumoxide::{browser::{Browser, BrowserConfig}, JsHtmlButtonElement, JsHtmlInputElement};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let (browser, mut handler, _process) = Browser::launch(BrowserConfig::builder().build()?).await?;

    let handle = async_std::task::spawn(async move {
        loop {
            let _ = handler.next().await.unwrap();
        }
    });

    let page = browser.new_page("https://en.wikipedia.org").await?;

    let input = page.query_selector("input#searchInput")
        .await?
        .expect("No input found")
        .downcast::<JsHtmlInputElement>()
        .expect("Input is not an HTML input element")
        ;
    input.scroll_into_view_if_needed().await?;
    input.click().await?;
    input.set_value("Rust (programming language)".to_string()).await?;

    let button = page.query_selector("#searchform > button")
        .await?
        .unwrap()
        .downcast::<JsHtmlButtonElement>()
        .unwrap()
        ;
    button.click().await?;

    let _html = page.wait_for_navigation().await?.content().await?;

    handle.await;
    Ok(())
}
