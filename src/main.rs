#![deny(warnings)]

extern crate shrust;
use shrust::{Shell, ShellIO};
use std::io::prelude::*;

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

    // warp::serve(warp::fs::dir("www"))
    //     .run(([127, 0, 0, 1], 3030))
    //     .await;

    let mut shell = Shell::new(());
    shell.new_command_noargs("hello", "Say 'hello' to the world", |io, _| {
        writeln!(io, "Hello World !!!")?;
        Ok(())
    });

    shell.run_loop(&mut ShellIO::default());
}