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
use subxt::{
    extrinsic::{PairSigner, Signer},
    ClientBuilder,
};

#[subxt::subxt(runtime_metadata_path = "metadata/substrate-node-chaos.scale")]
pub mod chaosrpc {}

pub async fn rpc_unwrap_add() -> Result<(), Box<dyn std::error::Error>> {
    // alice signer
    // let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let signer = PairSigner::<chaosrpc::DefaultConfig, _>::new(AccountKeyring::Alice.pair());

    // runtime API
    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<chaosrpc::RuntimeApi<chaosrpc::DefaultConfig>>();

    // submit extrinsic
    let result = api
        .tx()
        .chaos()
        .unwrap_add()
        .sign_and_submit_then_watch(&signer)
        .await?;

    // check event
    if result
        .find_event::<chaosrpc::chaos::events::Added>()?
        .is_none()
    {
        panic!("Event not found!");
    }

    Ok(())
}

pub async fn rpc_overflow_adder(n: u32) -> Result<u32, Box<dyn std::error::Error>> {
    // alice signer
    // let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let signer = PairSigner::<chaosrpc::DefaultConfig, _>::new(AccountKeyring::Alice.pair());

    // runtime API
    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<chaosrpc::RuntimeApi<chaosrpc::DefaultConfig>>();

    // submit extrinsic
    let result = api
        .tx()
        .chaos()
        .overflow_adder(n)
        .sign_and_submit_then_watch(&signer)
        .await?;

    // check event
    if result
        .find_event::<chaosrpc::chaos::events::Added>()?
        .is_none()
    {
        panic!("Event not found!");
    }

    let storage = api.storage().chaos();
    let adder = storage.adder(None).await.unwrap().unwrap();

    Ok(adder)
}

pub async fn rpc_clear_adder() -> Result<(), Box<dyn std::error::Error>> {
    // alice signer
    // let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let signer = PairSigner::<chaosrpc::DefaultConfig, _>::new(AccountKeyring::Alice.pair());

    // runtime API
    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<chaosrpc::RuntimeApi<chaosrpc::DefaultConfig>>();

    // submit extrinsic
    let result = api
        .tx()
        .chaos()
        .clear_adder()
        .sign_and_submit_then_watch(&signer)
        .await?;

    // check event
    if result
        .find_event::<chaosrpc::chaos::events::Stalled>()?
        .is_none()
    {
        panic!("Event not found!");
    }

    Ok(())
}

pub async fn rpc_drag_block_unit_weight(n: u64) -> Result<(), Box<dyn std::error::Error>> {
    // alice signer
    // let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let signer = PairSigner::<chaosrpc::DefaultConfig, _>::new(AccountKeyring::Alice.pair());

    // runtime API
    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<chaosrpc::RuntimeApi<chaosrpc::DefaultConfig>>();

    // get signer balance before
    let balance_before = api
        .storage()
        .system()
        .account(signer.account_id().clone(), None)
        .await?;

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

    // get signer balance after
    let balance_after = api
        .storage()
        .system()
        .account(signer.account_id().clone(), None)
        .await?;

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
    let fees_paid = balance_before.data.free - balance_after.data.free;

    println!(
        "n: {}, block execution time: {} ms, fees paid: {}",
        n,
        d.as_millis(),
        fees_paid
    );

    Ok(())
}
