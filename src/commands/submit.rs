use anyhow::Result;

pub async fn execute(problem_number: &char) -> Result<()> {
    println!("submit command! arg = {}", problem_number);

    Ok(())
}
