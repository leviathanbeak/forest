// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::load_state;
use address::Address;
use ipld_blockstore::BlockStore;
use serde::Serialize;
use std::error::Error;
use vm::ActorState;

/// Init actor address.
pub static ADDRESS: &actorv3::INIT_ACTOR_ADDR = &actorv3::INIT_ACTOR_ADDR;

/// Init actor method.
pub type Method = actorv3::init::Method;

/// Init actor state.
#[derive(Serialize)]
#[serde(untagged)]
pub enum State {
    V0(actorv0::init::State),
    V2(actorv2::init::State),
    V3(actorv3::init::State),
    V4(actorv4::init::State),
    V5(actorv5::init::State),
    V6(actorv6::init::State),
}

impl State {
    pub fn load<BS>(store: &BS, actor: &ActorState) -> anyhow::Result<State>
    where
        BS: BlockStore,
    {
        load_state!(
            store,
            actor,
            (actorv6::INIT_ACTOR_CODE_ID, State::V6),
            (actorv5::INIT_ACTOR_CODE_ID, State::V5),
            (actorv4::INIT_ACTOR_CODE_ID, State::V4),
            (actorv3::INIT_ACTOR_CODE_ID, State::V3),
            (actorv2::INIT_ACTOR_CODE_ID, State::V2),
            (actorv0::INIT_ACTOR_CODE_ID, State::V0)
        )
    }

    /// Allocates a new ID address and stores a mapping of the argument address to it.
    /// Returns the newly-allocated address.
    pub fn map_address_to_new_id<BS: BlockStore>(
        &mut self,
        store: &BS,
        addr: &Address,
    ) -> Result<Address, Box<dyn Error>> {
        match self {
            State::V0(st) => Ok(st.map_address_to_new_id(store, addr)?),
            State::V2(st) => Ok(st.map_address_to_new_id(store, addr)?),
            State::V3(st) => Ok(st.map_address_to_new_id(store, addr)?),
            State::V4(st) => Ok(st.map_address_to_new_id(store, addr)?),
            State::V5(st) => Ok(st.map_address_to_new_id(store, addr)?),
            State::V6(st) => Ok(st.map_address_to_new_id(store, addr)?),
        }
    }

    /// ResolveAddress resolves an address to an ID-address, if possible.
    /// If the provided address is an ID address, it is returned as-is.
    /// This means that mapped ID-addresses (which should only appear as values, not keys) and
    /// singleton actor addresses (which are not in the map) pass through unchanged.
    ///
    /// Returns an ID-address and `true` if the address was already an ID-address or was resolved
    /// in the mapping.
    /// Returns an undefined address and `false` if the address was not an ID-address and not found
    /// in the mapping.
    /// Returns an error only if state was inconsistent.
    pub fn resolve_address<BS: BlockStore>(
        &self,
        store: &BS,
        addr: &Address,
    ) -> Result<Option<Address>, Box<dyn Error>> {
        match self {
            State::V0(st) => st.resolve_address(store, addr),
            State::V2(st) => st.resolve_address(store, addr),
            State::V3(st) => st.resolve_address(store, addr),
            State::V4(st) => st.resolve_address(store, addr),
            State::V5(st) => st.resolve_address(store, addr),
            State::V6(st) => st.resolve_address(store, addr),
        }
    }

    pub fn into_network_name(self) -> String {
        match self {
            State::V0(st) => st.network_name,
            State::V2(st) => st.network_name,
            State::V3(st) => st.network_name,
            State::V4(st) => st.network_name,
            State::V5(st) => st.network_name,
            State::V6(st) => st.network_name,
        }
    }
}
