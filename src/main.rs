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
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Cmd {
    #[structopt(about = "Drags block production with unitary weight")]
    DragBlockUnitWeight {
        #[structopt(short = "n", default_value = "100_000_000")]
        n: u64,
    },
}

#[derive(Debug, StructOpt)]
enum Opt {
    #[structopt(flatten)]
    Cmd(Cmd),
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    match Cmd::from_args() {
        Cmd::DragBlockUnitWeight { n } => {
            let rpc_future = chaoscope::rpc_drag_block_unit_weight(n);
            match futures::executor::block_on(rpc_future) {
                Ok(r) => r,
                Err(e) => {
                    panic!("err: {}", e);
                }
            };
        }
    }
}
