mod test;
use test::Example;
struct Implementor;

#[async_trait::async_trait]
impl Example for Implementor {
    async fn test(&self) -> u32 {
        4
    } 
}

fn main() {
    println!("Hello, world!");
}
