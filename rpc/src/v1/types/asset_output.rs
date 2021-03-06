// Copyright 2018-2019 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::convert::TryFrom;
use std::iter::FromIterator;
use std::ops::Deref;

use cjson::uint::Uint;
use ctypes::transaction::{AssetMintOutput as AssetMintOutputType, AssetTransferOutput as AssetTransferOutputType};
use ctypes::ShardId;
use primitives::H160;
use rustc_serialize::hex::{FromHex, FromHexError, ToHex};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetTransferOutput {
    pub lock_script_hash: H160,
    pub parameters: Vec<String>,
    pub asset_type: H160,
    pub shard_id: ShardId,
    pub quantity: Uint,
}

impl From<AssetTransferOutputType> for AssetTransferOutput {
    fn from(from: AssetTransferOutputType) -> Self {
        AssetTransferOutput {
            lock_script_hash: from.lock_script_hash,
            parameters: from.parameters.iter().map(Deref::deref).map(<[u8]>::to_hex).collect(),
            asset_type: from.asset_type,
            shard_id: from.shard_id,
            quantity: from.quantity.into(),
        }
    }
}

impl TryFrom<AssetTransferOutput> for AssetTransferOutputType {
    type Error = FromHexError;
    fn try_from(from: AssetTransferOutput) -> Result<Self, Self::Error> {
        Ok(AssetTransferOutputType {
            lock_script_hash: from.lock_script_hash,
            parameters: Result::from_iter(from.parameters.iter().map(Deref::deref).map(FromHex::from_hex))?,
            asset_type: from.asset_type,
            shard_id: from.shard_id,
            quantity: from.quantity.into(),
        })
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetMintOutput {
    pub lock_script_hash: H160,
    pub parameters: Vec<String>,
    pub supply: Uint,
}

impl From<AssetMintOutputType> for AssetMintOutput {
    fn from(from: AssetMintOutputType) -> Self {
        AssetMintOutput {
            lock_script_hash: from.lock_script_hash,
            parameters: from.parameters.iter().map(Deref::deref).map(<[u8]>::to_hex).collect(),
            supply: from.supply.into(),
        }
    }
}

impl TryFrom<AssetMintOutput> for AssetMintOutputType {
    type Error = FromHexError;
    fn try_from(from: AssetMintOutput) -> Result<Self, Self::Error> {
        Ok(AssetMintOutputType {
            lock_script_hash: from.lock_script_hash,
            parameters: Result::from_iter(from.parameters.iter().map(Deref::deref).map(FromHex::from_hex))?,
            supply: from.supply.into(),
        })
    }
}
