use cryptopals::set1::solve_challenge1;
use cryptopals::set1::solve_challenge2;
use cryptopals::set1::solve_challenge3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Cryptopals Challenges ===\n");
    
    // Run Challenge 1
    solve_challenge1()?;
    println!();
    
    solve_challenge2()?;
    println!();
    
    solve_challenge3()?;
    
    Ok(())
}
