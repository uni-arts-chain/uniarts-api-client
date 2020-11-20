/*
    Copyright 2019 Supercomputing Systems AG
    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

///! Very simple example that shows how to use a predefined extrinsic from the extrinsic module
use clap::{load_yaml, App};
use keyring::AccountKeyring;
use sp_core::crypto::Pair;
use nft;

use substrate_api_client::{Api, XtStatus};

fn main() {
    env_logger::init();
    let url = get_node_url_from_cli();

    // initialize api and set the signer (sender) that is used to sign the extrinsics
    let from = AccountKeyring::Alice.pair();
    let api = Api::new(url).set_signer(from.clone());

    // generate extrinsic
    let nft_name = vec![0u16, 0u16, 1u16];
    let nft_description = vec![0u16, 0u16, 1u16];
    let token_prefix = vec![0u8, 0u8, 1u8];
    let mode = nft::CollectionMode::NFT(4096u32);
    let xt = api.nft_create_collection(nft_name, nft_description, token_prefix, mode);

    println!(
        "Sending an nft_create_collection extrinsic from Alice (Key = {}),\n\n",
        from.public(),
    );
    println!("[+] Composed extrinsic: {:?}\n", xt);

    // send and watch extrinsic until finalized
    let tx_hash = api
        .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
        .unwrap();
    println!("[+] Transaction got included. Hash: {:?}\n", tx_hash);


    // generate extrinsic
    let collection_id = 1u64;
    let item_properties = vec![22u8, 33u8, 44u8];
    let to = AccountKeyring::Bob.to_account_id();

    let item_xt = api.nft_create_item(collection_id, item_properties, to);

    println!(
        "Sending an nft_create_item extrinsic from Alice (Key = {}),\n\n",
        from.public(),
    );
    println!("[+] Composed extrinsic: {:?}\n", item_xt);

    // send and watch extrinsic until finalized
    let tx_hash = api
        .send_extrinsic(item_xt.hex_encode(), XtStatus::InBlock)
        .unwrap();
    println!("[+] Transaction got included. Hash: {:?}\n", tx_hash);

}

pub fn get_node_url_from_cli() -> String {
    let yml = load_yaml!("../../src/examples/cli.yml");
    let matches = App::from_yaml(yml).get_matches();

    let node_ip = matches.value_of("node-server").unwrap_or("ws://127.0.0.1");
    let node_port = matches.value_of("node-port").unwrap_or("9944");
    let url = format!("{}:{}", node_ip, node_port);
    println!("Interacting with node on {}\n", url);
    url
}
