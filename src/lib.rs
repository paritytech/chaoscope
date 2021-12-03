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

// chaoscope RPC calls

use sp_keyring::AccountKeyring;
use std::time::Duration;
use subxt::{ClientBuilder, PairSigner};

#[subxt::subxt(runtime_metadata_path = "metadata/substrate-node-chaos.scale")]
pub mod chaosrpc {}

pub async fn rpc_drag_block_unit_weight(n: u64) -> Result<(), Box<dyn std::error::Error>> {
    // alice signer
    let signer = PairSigner::new(AccountKeyring::Alice.pair());

    // runtime API
    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<chaosrpc::RuntimeApi<chaosrpc::DefaultConfig>>();

    // block subscriber
    let mut sub = api.client.rpc().subscribe_blocks().await.unwrap();

    // get head block hash
    let head_block_hash = loop {
        if let Ok(Some(block)) = sub.next().await {
            break block.hash();
        }
    };

    // get head block timestamp
    let head_block_before = api
        .storage()
        .timestamp()
        .now(Some(head_block_hash))
        .await
        .unwrap();

    // submit extrinsic
    let result = api
        .tx()
        .chaos()
        .drag_block_unit_weight(n)
        .sign_and_submit_then_watch(&signer)
        .await?;

    // check event
    if result
        .find_event::<chaosrpc::chaos::events::Stalled>()?
        .is_none()
    {
        panic!("Event not found!");
    }

    // get head block timestamp
    let head_block_hash = loop {
        if let Ok(Some(block)) = sub.next().await {
            break block.hash();
        }
    };

    // get head block timestamp
    let head_block_after = api
        .storage()
        .timestamp()
        .now(Some(head_block_hash))
        .await
        .unwrap();

    // calculate block time (after - before)
    let block_time = head_block_after.checked_sub(head_block_before).unwrap();
    let d = Duration::from_millis(block_time);

    // get fees paid

    println!("n: {}, block execution time: {} ms", n, d.as_millis());

    Ok(())
}
