use chromiumoxide::browser::{Browser, BrowserConfig};
use futures::StreamExt;
use chromiumoxide::cdp::browser_protocol::target::CreateBrowserContextParams;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let (browser, mut handler, _process) =
        Browser::launch(BrowserConfig::builder().with_head().build()?).await?;

    let handle = tokio::spawn(async move {
        loop {
            let _ = handler.next().await.unwrap();
        }
    });

    let params = CreateBrowserContextParams::builder().build();
    // switch to incognito mode and goto the url
    let _incognito_page = browser
        .create_browser_context(params)
        .await?
        .new_page("https://en.wikipedia.org")
        .await?;

    handle.await?;
    Ok(())
}
