use anyhow::Result;
use scraper::{Html, Selector};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[tokio::main]
async fn main() -> Result<()> {
    let response = reqwest::get("https://kopps.com").await.unwrap();

    let html = Html::parse_document(&response.text().await.unwrap());
    let selector = Selector::parse(r#".display-1"#).unwrap();

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout
        .set_color(ColorSpec::new().set_bold(true).set_fg(Some(Color::Magenta)))
        .unwrap();

    writeln!(&mut stdout, "__TODAY__")?;

    stdout
        .set_color(ColorSpec::new().set_bold(false).set_fg(None))
        .unwrap();
    for flavor in html.select(&selector) {
        println!(
            "🍦 {}",
            flavor.text().next().unwrap().parse::<String>().unwrap()
        );
    }

    stdout
        .set_color(ColorSpec::new().set_bold(true).set_fg(Some(Color::Cyan)))
        .unwrap();
    println!("\n__TOMORROW__");

    let response = reqwest::get("https://kopps.com/flavor-preview")
        .await
        .unwrap();

    let html = Html::parse_document(&response.text().await.unwrap());
    let selector = Selector::parse(r#".h5"#).unwrap();

    stdout
        .set_color(ColorSpec::new().set_bold(false).set_fg(None))
        .unwrap();

    writeln!(
        &mut stdout,
        "🍦 {}",
        html.select(&selector)
            .nth(8)
            .unwrap()
            .text()
            .next()
            .unwrap()
            .parse::<String>()
            .unwrap()
    )?;

    writeln!(
        &mut stdout,
        "🍦 {}",
        html.select(&selector)
            .nth(9)
            .unwrap()
            .text()
            .next()
            .unwrap()
            .parse::<String>()
            .unwrap()
    )?;

    Ok(())
}
