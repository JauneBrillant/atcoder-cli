use anyhow::Result;

pub async fn execute(problem_number: &char) -> Result<()> {
    println!("test command!, arg = {}", problem_number);

    Ok(())
}
