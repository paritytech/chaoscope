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
// You should have received a copy of the GNU General Public License
// along with chaoscope.  If not, see <http://www.gnu.org/licenses/>.

use subxt::ClientBuilder;

#[subxt::subxt(runtime_metadata_path = "examples/substrate-node-chaos.scale")]
pub mod block_dragger {}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let api = ClientBuilder::new()
        .set_url("wss://localhost:9944")
        .build()
        .await?
        .to_runtime_api::<block_dragger::RuntimeApi<block_dragger::DefaultConfig>>();

    Ok(())
}
