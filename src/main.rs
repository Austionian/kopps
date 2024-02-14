use anyhow::Result;
use scraper::{Html, Selector};
use spinners::{Spinner, Spinners};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[tokio::main]
async fn main() -> Result<()> {
    get_today().await?;
    get_tomorrow().await?;
    Ok(())
}

async fn get_today() -> Result<()> {
    let mut spinner = Spinner::new(Spinners::Dots, "Loading the flavor forecast...".into());
    let response = reqwest::get("https://kopps.com").await.unwrap();

    let html = Html::parse_document(&response.text().await.unwrap());
    let selector = Selector::parse(r#".display-1"#).unwrap();

    spinner.stop_with_newline();
    std::process::Command::new("clear").status().unwrap();

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

    Ok(())
}

async fn get_tomorrow() -> Result<()> {
    println!("\n__TOMORROW__");
    let response = reqwest::get("https://kopps.com/flavor-preview")
        .await
        .unwrap();

    let html = Html::parse_document(&response.text().await.unwrap());
    let selector = Selector::parse(r#".h5"#).unwrap();

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
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
