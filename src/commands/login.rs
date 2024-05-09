use anyhow::{Context, Result};
use promptuity::{
    prompts::{Input, Password},
    themes::FancyTheme,
    Promptuity, Term,
};

pub fn execute() {
    match get_credentials() {
        Ok((name, pass)) => {
            println!("username = {:?}, password = {:?}", name, pass);
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn get_credentials() -> Result<(String, String)> {
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
