#![deny(warnings)]

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    println!("          _");
    println!("         ⇖⇑⇗");
    println!("         ⇐●⇒");
    println!("         ⇙⇓⇘");
    println!("          ‾");
    println!("⚠️Expect... Chaoscope! ⚠");
    println!("Open http://localhost:3030 on your browser.");

    warp::serve(warp::fs::dir("www"))
        .run(([127, 0, 0, 1], 3030))
        .await;
}