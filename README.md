A simple Rust library for simulating a high cpu load.
You can stress your CPU by specifying the number of workers
and timeout in seconds. Each worker corresponds a native OS thread.
For example,

```Rust
#[tokio::main]
async fn main() {
    stress_cpu::stress(4, 30).await;
}
```

