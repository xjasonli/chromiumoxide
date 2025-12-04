use chromiumoxide::browser::{Browser, BrowserConfig};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (browser, mut handler, _process) = Browser::launch(BrowserConfig::builder().build()?).await?;

    let handle = tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            match h {
                Ok(_) => continue,
                Err(_) => break,
            }
        }
    });

    let page = browser.new_page("about:blank").await?;

    let sum: usize = page.eval("1 + 2").await?;
    assert_eq!(sum, 3);
    println!("1 + 2 = {sum}");

    let mult: usize = page
        .invoke_function("() => { return 21 * 2; }")
        .invoke()
        .await?;
    assert_eq!(mult, 42);
    println!("21 * 2 = {mult}");

    let promise_div: usize = page
        .invoke_function("() => Promise.resolve(100 / 25)")
        .invoke()
        .await?;
    assert_eq!(promise_div, 4);
    println!("100 / 25 = {promise_div}");

    let sum: usize = page.invoke_function("(a,b) => {return a + b;}")
        .arguments((1, 2))
        .invoke().await?;
    assert_eq!(sum, 3);
    println!("1 + 2 = {sum}");

    let sum: usize = page
        .eval("((a,b) => {return a + b;})(1,2)")
        .await?;
    assert_eq!(sum, 3);
    println!("1 + 2 = {sum}");

    let val: usize = page
        .invoke_function("async function() {return 42;}")
        .invoke()
        .await?;
    assert_eq!(val, 42);
    println!("42 = {val}");

    browser.close().await?;
    handle.await?;
    Ok(())
}
