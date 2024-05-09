use promptuity::{
    prompts::{Input, Password},
    themes::FancyTheme,
    Promptuity, Term,
};

pub fn execute() {
    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    let _ = p.term().clear();
    let _ = p.with_intro("Login AtCoder!").begin();

    let name: Result<String, promptuity::Error> =
        p.prompt(Input::new("Please enter your username").with_placeholder("username"));
    let pass: Result<String, promptuity::Error> =
        p.prompt(Password::new("Please enter your password").with_required(false));

    println!("username = {:?}, password = {:?}", name, pass);
}
