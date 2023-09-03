use scraper::{Html, Selector};

#[tokio::main]
async fn main() {
    let response = reqwest::get("https://kopps.com").await.unwrap();

    let html = Html::parse_document(&response.text().await.unwrap());
    let selector = Selector::parse(r#".display-1"#).unwrap();

    println!("__TODAY__");

    for flavor in html.select(&selector) {
        println!(
            "{}",
            flavor.text().next().unwrap().parse::<String>().unwrap()
        );
    }

    println!("\n__TOMORROW__");

    let response = reqwest::get("https://kopps.com/flavor-preview")
        .await
        .unwrap();

    let html = Html::parse_document(&response.text().await.unwrap());
    let selector = Selector::parse(r#".h5"#).unwrap();

    println!(
        "{}",
        html.select(&selector)
            .nth(8)
            .unwrap()
            .text()
            .next()
            .unwrap()
            .parse::<String>()
            .unwrap()
    );

    println!(
        "{}",
        html.select(&selector)
            .nth(9)
            .unwrap()
            .text()
            .next()
            .unwrap()
            .parse::<String>()
            .unwrap()
    );
}
