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
use chaoscope;
use futures;
use shrust::{Shell, ShellIO};
use std::{io::prelude::*, net::TcpListener};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    println!("          _");
    println!("         ⇖⇑⇗");
    println!("         ⇐●⇒");
    println!("         ⇙⇓⇘");
    println!("          ‾");
    println!("⚠️Expect... Chaoscope Shell! ⚠");
    println!("A TCP socket is listening as Chaoscope Shell at port 1234.");
    println!("You can connect to it from another machine by typing \"nc x.y.w.z 1234\" on a new terminal.");
    println!("Type \"help\" to learn how to interact with Chaoscope Shell.");

    let mut shell = Shell::new(());

    shell.new_command("drag_block_unit_weight", "Drags block production by calculating hashes in a loop (n times). \nUses constant unitary extrinsic weight. \nExpected args: n \nUsage example: drag_block_unit_weight 10000000",
                      1 ,
                      |io,
                       _,
                       args| {
        let n = match args[0].parse::<u32>() {
            Ok(n) => n,
            Err(_) => { writeln!(io, "n must be integer!")?; 0 },
        };

        writeln!(io, "n = {}", n)?;

        let rpc_future = chaoscope::rpc_drag_block_unit_weight(10_000_000);

        let ret = match futures::executor::block_on(rpc_future) {
            Ok(r) => r,
            Err(e) => { println!("err: {}", e); 0 },
        };

        writeln!(io, "{}", ret)?;
        Ok(())
    });

    shell.new_command("drag_block_damp_weight", "Drags block production by calculating hashes in a loop (n times). \nUses linear damping on weight (0.0 < wd < 1.0). \nExpected args: n wd \nUsage example: drag_block_damp_weight 10000000 0.01", 2 ,|io, _, args|  {
        let n = match args[0].parse::<u32>() {
            Ok(n) => n,
            Err(_) => { writeln!(io, "n must be integer!")?; 0 },
        };

        let wd = match args[1].parse::<f32>() {
            Ok(wd) => wd,
            Err(_) => { writeln!(io, "wd must be float!")?; 0.0 },
        };

        writeln!(io, "n = {}", n)?;
        writeln!(io, "wd = {:.2}", wd)?;
        Ok(())
    });

    let mut shell_io = shell.clone();
    tokio::task::spawn_blocking(move || shell_io.run_loop(&mut ShellIO::default()));

    let serv = TcpListener::bind("0.0.0.0:1234").expect("Cannot open socket");
    for sock in serv.incoming() {
        let sock = sock.unwrap();
        let mut shell = shell.clone();
        let mut io = ShellIO::new_io(sock);
        writeln!(io, "          _").unwrap();
        writeln!(io, "         ⇖⇑⇗").unwrap();
        writeln!(io, "         ⇐●⇒").unwrap();
        writeln!(io, "         ⇙⇓⇘").unwrap();
        writeln!(io, "          ‾").unwrap();
        writeln!(io, "⚠️Expect... Chaoscope Shell! ⚠").unwrap();
        writeln!(
            io,
            "Type \"help\" to learn how to interact with Chaoscope Shell."
        )
        .unwrap();
        tokio::task::spawn_blocking(move || shell.run_loop(&mut io));
    }
}
