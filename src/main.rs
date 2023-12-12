use nanopost::*;
fn main() {
    build(process_html);

    let args = std::env::args().collect::<Vec<String>>();

    if args.len() > 1 && args[1] == "dev" {
        serve();
    }
}

fn process_html(content: &str) -> String {
    (r#"<!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                </head>
                <body>"#
        .to_owned()
        + content)
        .to_owned()
        + r#"</body>
                </html>"#
}
