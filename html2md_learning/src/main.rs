use std::fs;

async fn html_to_md() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("Fetching url: {}", url);
    let body: Result<reqwest::Response, reqwest::Error> = reqwest::get("https://www.rust-lang.org")
        .await;

    match body {
        Ok(res) => {
            println!("Converting html to markdown...");
            if let Ok(res_str) = res.text().await {

                let md = html2md::parse_html(&res_str);

                match fs::write(output, md.as_bytes()) {
                    Ok(_) => println!("Converted markdown has been in {}.", output),
                    Err(err) => {
                        println!("Converting html to markdown error: {}", err);
                    }
                };
            } else {
                println!("Converting html to markdown error");
            }
        }
        Err(err) => {
            println!("Converting html to markdown error: {}", err);
        }
    }
}

#[tokio::main]
async fn main() {
    html_to_md().await;
}
