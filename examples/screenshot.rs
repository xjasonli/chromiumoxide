use futures::StreamExt;

use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::page::ScreenshotParams;
use chromiumoxide_cdp::cdp::browser_protocol::page::CaptureScreenshotFormat;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let (browser, mut handler, _process) = Browser::launch(BrowserConfig::builder().build()?).await?;

    let handle = async_std::task::spawn(async move {
        loop {
            let _ = handler.next().await.unwrap();
        }
    });

    let page = browser.new_page("https://news.ycombinator.com/").await?;

    // take a screenshot of the page
    page.save_screenshot(
        ScreenshotParams::builder()
            .format(CaptureScreenshotFormat::Png)
            .full_page(true)
            .omit_background(true)
            .build(),
        "hn-page.png",
    )
    .await?;

    // get the top post and save a screenshot of it
    let image = page.query_selector("table.itemlist tr")
        .await?
        .expect("No top post found")
        .screenshot(CaptureScreenshotFormat::Png)
        .await?;

    std::fs::write("top-post.png", image)?;

    handle.await;
    Ok(())
}
