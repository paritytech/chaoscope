// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of chaoscope.
//
// chaoscope is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// chaoscope is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the Affero GNU General Public License
// along with chaoscope.  If not, see <http://www.gnu.org/licenses/>.

extern crate shrust;
use shrust::{Shell, ShellIO};
use std::io::prelude::*;

use chaoscope;

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
        // writeln!(io, "Hello World !!!")?;
        let result = chaoscope::rpc_drag_block_unit_weight(10_000_000);
        // ...

        writeln!(io, "Hello drag_block_unit_weight !!!")?;
        Ok(())
    });

    shell.new_command_noargs("drag_block_damp_weight", "Drags block production on the runtime by calculating hashes in a loop (n times). Uses linear damping on weight (`0.0 < wd < 1.0`).", |io, _| {
        writeln!(io, "Hello drag_block_damp_weight !!!")?;
        Ok(())
    });

    shell.run_loop(&mut ShellIO::default());
}
