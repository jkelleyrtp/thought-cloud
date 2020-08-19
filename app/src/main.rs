use web_view::*;

fn main() {
    web_view::builder()
        .title("todomvc example")
        .content(Content::Url(format!("http://127.0.0.1:{}/app", 8000)))
        .size(1600, 1000)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
