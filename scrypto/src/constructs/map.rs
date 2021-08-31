use sbor::*;

use crate::buffer::*;
use crate::kernel::*;
use crate::types::*;
use crate::utils::*;

#[derive(Debug, Encode, Decode, Describe)]
pub struct Map {
    mid: MID,
}

impl From<MID> for Map {
    fn from(mid: MID) -> Self {
        Self { mid }
    }
}

impl Into<MID> for Map {
    fn into(self) -> MID {
        self.mid
    }
}

impl Map {
    pub fn new() -> Self {
        let input = CreateMapInput {};
        let output: CreateMapOutput = call_kernel(CREATE_MAP, input);

        output.map.into()
    }

    pub fn get_entry<K: Encode, V: Decode>(&self, key: K) -> Option<V> {
        let input = GetMapEntryInput {
            map: self.mid,
            key: scrypto_encode(&key),
        };
        let output: GetMapEntryOutput = call_kernel(GET_MAP_ENTRY, input);

        output.value.map(|v| unwrap_or_panic(scrypto_decode(&v)))
    }

    pub fn put_entry<K: Encode, V: Encode>(&self, key: K, value: V) {
        let input = PutMapEntryInput {
            map: self.mid,
            key: scrypto_encode(&key),
            value: scrypto_encode(&value),
        };
        let _: PutMapEntryOutput = call_kernel(PUT_MAP_ENTRY, input);
    }

    pub fn mid(&self) -> MID {
        self.mid
    }
}