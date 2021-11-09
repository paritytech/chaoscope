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

    // stdout via browser
    tokio::spawn(async move {
        warp::serve(warp::fs::dir("www"))
            .run(([127, 0, 0, 1], 3030))
            .await;
    });

    let mut shell = Shell::new(());

    shell.new_command_noargs("drag_block_unit_weight", "Drags block production on the runtime by calculating hashes in a loop (n times). Uses constant unitary extrinsic weight.", |io, _| {
        writeln!(io, "Hello World !!!")?;
        Ok(())
    });

    shell.new_command_noargs("drag_block_damp_weight", "Drags block production on the runtime by calculating hashes in a loop (n times). Uses linear damping on weight (`0.0 < wd < 1.0`).", |io, _| {
        writeln!(io, "Hello World !!!")?;
        Ok(())
    });

    shell.run_loop(&mut ShellIO::default());
}
