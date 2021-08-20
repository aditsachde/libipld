#![allow(warnings)]
//! CBOR encoder and decoder for BitVec
use crate::decode::{read_len, read_list, read_list_il, read_u8};
use crate::encode::write_u64;
use crate::error::{UnexpectedCode, UnexpectedKey};
use crate::DagCborCodec as DagCbor;
use bitvec::prelude::*;
use libipld_core::codec::{Decode, Encode};

impl Decode<DagCbor> for BitVec<Msb0, u8> {
    fn decode<R: std::io::Read + std::io::Seek>(
        c: DagCbor,
        r: &mut R,
    ) -> libipld_core::error::Result<Self> {
        let major = read_u8(r)?;
        let result: Vec<u8> = match major {
            0x80..=0x9b => {
                let len = read_len(r, major - 0x80)?;
                read_list(r, len)?
            }
            0x9f => read_list_il(r)?,
            _ => {
                return Err(UnexpectedCode::new::<Self>(major).into());
            }
        };
        let result = BitVec::<Msb0, u8>::try_from_vec(result)
            .map_err(|op| UnexpectedKey::new::<BitVec<Msb0, u8>>(String::from("Conversion failed from u8 vec")))?;
        Ok(result)
    }
}

impl Encode<DagCbor> for BitVec<Msb0, u8> {
    fn encode<W: std::io::Write>(&self, c: DagCbor, w: &mut W) -> libipld_core::error::Result<()> {
        let slice = self.as_raw_slice();
        write_u64(w, 4, (self.len() / 8) as u64)?;

        for n in 0..(self.len() / 8) {
            slice.get(n).encode(c, w)?
        }
        Ok(())
    }
}
