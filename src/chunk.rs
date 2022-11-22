use glam::IVec2;
use internment::Intern;
use serde::Deserialize;
use std::{collections::BTreeMap, io::Cursor, ops::Range};

lazy_static::lazy_static! {
    pub static ref AIR: Intern<Block> = Intern::new(Block {
        name: Intern::from_ref("minecraft:air"),
        properties: Intern::default(),
    });
}

#[derive(Deserialize)]
pub struct Chunk {
    pub sections: [Section; 24],
}

impl Chunk {
    pub fn load(
        file: &[u8],
        area: Range<IVec2>,
        x: i32,
        z: i32,
    ) -> Option<Self> {
        if x * 16 + 15 < area.start.x
            || x * 16 >= area.end.x
            || z * 16 + 15 < area.start.y
            || z * 16 >= area.end.y
        {
            return None;
        }
        let x = x.rem_euclid(32) as usize;
        let z = z.rem_euclid(32) as usize;
        let locations: &[[u8; 4]] = bytemuck::cast_slice(&file[..4096]);
        let raw_location = locations[z * 32 + x];
        let data_offset = u32::from_be_bytes([
            0,
            raw_location[0],
            raw_location[1],
            raw_location[2],
        ]) as usize
            * 4096;
        if data_offset == 0 {
            return None;
        }
        let payload = &file[data_offset..];
        let length =
            u32::from_be_bytes(bytemuck::cast_slice(payload)[0]) as usize - 1;
        let data = &payload[5..][..length];
        let compression_scheme = payload[4];
        assert_eq!(
            compression_scheme, 2,
            "only zlib chunk compression is supported"
        );
        Some(nbt::from_zlib_reader(&mut Cursor::new(data)).unwrap())
    }
}

#[derive(Deserialize)]
pub struct Section {
    #[serde(default)]
    pub block_states: BlockStates,
    #[serde(default)]
    #[serde(rename = "BlockLight")]
    pub block_light: Box<[i8]>,
    #[serde(default)]
    #[serde(rename = "SkyLight")]
    pub sky_light: Box<[i8]>,
}

pub struct BlockStates {
    pub palette: Vec<Intern<Block>>,
    pub data: Box<[u16; 4096]>,
}

impl Default for BlockStates {
    fn default() -> Self {
        Self {
            palette: vec![*AIR],
            data: Box::new([0; 4096]),
        }
    }
}

impl<'de> Deserialize<'de> for BlockStates {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct DeBlockStates {
            palette: Vec<Intern<Block>>,
            data: Option<Vec<i64>>,
        }

        let DeBlockStates { palette, data } =
            DeBlockStates::deserialize(deserializer)?;
        if palette.len() == 1 {
            Ok(Self {
                palette,
                data: Box::new([0; 4096]),
            })
        } else {
            let data = data.unwrap();
            let index_bit_length =
                (usize::BITS - (palette.len() - 1).leading_zeros()).max(4);
            let indices_per_long = 64 / index_bit_length;
            let mask = (1u64 << index_bit_length) - 1;
            assert!(indices_per_long as usize * data.len() >= 4096,);
            let data = data
                .into_iter()
                .flat_map(|long| {
                    (0..indices_per_long).map(move |i| {
                        ((long as u64 >> (i * index_bit_length)) & mask) as u16
                    })
                })
                .take(4096)
                .collect::<Box<_>>()
                .try_into()
                .unwrap();

            Ok(Self { palette, data })
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Block {
    pub name: Intern<Box<str>>,
    #[serde(default)]
    pub properties: Intern<BTreeMap<String, String>>,
}
