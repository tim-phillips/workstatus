mod gh;
mod model;

use anyhow::Result;

fn main() -> Result<()> {
    gh::check_gh_available()?;
    let repo = gh::check_repo_context()?;
    let prs = gh::list_prs(100)?;
    println!("{repo}: {} open PR(s)", prs.len());
    for pr in &prs {
        println!(
            "  #{:<5} {:?} {:?} {}",
            pr.number, pr.review, pr.checks.overall, pr.title
        );
    }
    Ok(())
}
