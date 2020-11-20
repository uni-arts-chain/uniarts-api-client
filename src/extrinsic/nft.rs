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

use codec::Compact;

use super::xt_primitives::*;
#[cfg(feature = "std")]
use crate::{compose_extrinsic, Api};
use sp_core::crypto::Pair;
use sp_runtime::MultiSignature;
use nft;

pub const NFT_MODULE: &str = "Nft";
pub const NFT_CREATE_COLLECTION: &str = "create_collection";
pub const NFT_CREATE_ITEM: &str = "create_item";

pub type CallIndex = [u8; 2];
pub type Balance = u128;
pub type NextId = u64;
pub type ModeId = u8;

pub type NftCreateCollectionFn = (
    CallIndex,
    Vec<u16>,
    Vec<u16>,
    Vec<u8>,
    nft::CollectionMode,
);
pub type NftCreateCollectionXt = UncheckedExtrinsicV4<NftCreateCollectionFn>;

#[cfg(feature = "std")]
impl<P> Api<P>
    where
        P: Pair,
        MultiSignature: From<P::Signature>,
{
    pub fn nft_create_collection(&self, collection_name: Vec<u16>, collection_description: Vec<u16>, token_prefix: Vec<u8>, mode: nft::CollectionMode) -> NftCreateCollectionXt {
        compose_extrinsic!(
            self,
            NFT_MODULE,
            NFT_CREATE_COLLECTION,
            collection_name,
            collection_description,
            token_prefix,
            mode
        )
    }
}
