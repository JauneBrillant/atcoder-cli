use std::sync::Arc;

use anyhow::{Context, Result};
use promptuity::{
    prompts::{Input, Password},
    themes::FancyTheme,
    Promptuity, Term,
};
use reqwest::{cookie::Jar, Client};
use scraper::{Html, Selector};

pub async fn execute() -> Result<()> {
    let cookie_store = Arc::new(Jar::default());
    let client = Client::builder()
        .cookie_store(true)
        .cookie_provider(Arc::clone(&cookie_store))
        .build()?;

    let (username, password) = get_username_and_password_from_prompt()?;
    let res = login(&client, &username, &password).await?;
    println!("res = {}", res);

    Ok(())
}

fn get_username_and_password_from_prompt() -> Result<(String, String)> {
    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    let _ = p.term().clear();
    let _ = p.with_intro("Login AtCoder!").begin();

    let name = p
        .prompt(Input::new("Please enter your username").with_placeholder("username"))
        .context("Failed to get username")?;
    let pass = p
        .prompt(Password::new("Please enter your password").with_required(false))
        .context("Failed to get password")?;

    Ok((name, pass))
}

async fn login(client: &Client, username: &str, password: &str) -> Result<String> {
    let response = client.get("https://atcoder.jp/login").send().await?;
    let html = response.text().await?;
    let document = Html::parse_document(&html);

    let csrf_token_selector = Selector::parse("input[name='csrf_token']").unwrap();
    let csrf_token_element = document
        .select(&csrf_token_selector)
        .next()
        .ok_or_else(|| anyhow::anyhow!("CSRF token not found"))?;

    let csrf_token = csrf_token_element
        .value()
        .attr("value")
        .ok_or_else(|| anyhow::anyhow!("CSRF token value not found"))?
        .to_string();

    let response = client
        .post("https://atcoder.jp/login")
        .form(&[
            ("username", username),
            ("password", password),
            ("csrf_token", &csrf_token),
        ])
        .send()
        .await
        .context("Failed to send POST request")?;

    let text = response
        .text()
        .await
        .context("Failed to get response text")?;

    Ok(text)
}
