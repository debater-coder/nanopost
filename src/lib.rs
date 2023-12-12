use std::fs;

pub fn build<F>(process_html: F)
where
    F: Fn(&str) -> String,
{
    let cwd = std::env::current_dir().unwrap();
    let src = cwd.join("src/");
    let dist = cwd.join("dist/");

    fs::remove_dir_all(&dist).unwrap_or(());
    fs::create_dir(&dist).unwrap();

    // Copy src -> dist
    for entry in fs::read_dir(&src).unwrap() {
        let filename = &entry.unwrap().file_name();
        let filename = filename.to_str().unwrap();
        let extension = filename.split('.').last();

        match extension {
            Some("html") => {
                let content = &fs::read_to_string(&src.join(filename)).unwrap();
                let content = process_html(content.as_str());
                fs::write(&dist.join(filename), content).unwrap();
            }
            Some("md") => {
                let content = &fs::read_to_string(&src.join(filename)).unwrap();
                let content = markdown::to_html(content.as_str());
                let content = process_html(content.as_str());
                fs::write(&dist.join(filename.replace(".md", ".html")), content).unwrap();
            }
            _ => {
                fs::copy(&src.join(filename), &dist.join(filename)).unwrap();
            }
        }
    }
}

pub fn serve() {
    let cwd = std::env::current_dir().unwrap();
    let dist = cwd.join("dist/");

    rouille::start_server("localhost:8000", move |request| {
        if request.url() == "/" {
            // List all files in dist
            let mut content = String::new();
            for entry in fs::read_dir(&dist).unwrap() {
                let filename = &entry.unwrap().file_name();
                let filename = filename.to_str().unwrap();
                content += format!("<a href=\"{}\">{}</a><br>", filename, filename).as_str();
            }
            return rouille::Response::html(content);
        }
        rouille::match_assets(&request, &dist)
    });
}
