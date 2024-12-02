extern crate stress_cpu;

#[tokio::main]
async fn main() {
    stress_cpu::stress(4, 30).await;
}
