use futures::StreamExt;

use chromiumoxide::{browser::{Browser, BrowserConfig}, cdp::browser_protocol::target::CreateBrowserContextParams};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let (browser, mut handler, _process) =
        Browser::launch(BrowserConfig::builder().with_head().build()?).await?;

    let handle = async_std::task::spawn(async move {
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

    handle.await;
    Ok(())
}
