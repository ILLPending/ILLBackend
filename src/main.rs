// TODO: read this from env
const PORT: u16 = 3001;

#[tokio::main]
async fn main() {
    ill_backend_core::run(PORT)
        .await
        .expect("Fatal error while starting API");
}
