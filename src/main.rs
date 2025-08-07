use cryptopals::set1::solve_challenge1;
use cryptopals::set1::solve_challenge2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Cryptopals Challenges ===\n");
    
    // Run Challenge 1
    solve_challenge1()?;
    solve_challenge2()?;
    
    Ok(())
}
