use std::io::Result;
mod alg;
fn main() -> Result<()> {
    alg::uf::run();
    Ok(())
}
