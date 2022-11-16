use internment::Intern;
use serde::Deserialize;
use std::io::Cursor;

#[derive(Deserialize)]
pub struct Chunk {
    pub sections: [Section; 24],
}

impl Chunk {
    pub fn load(file: &[u8], x: usize, z: usize) -> Option<Self> {
        if x >= 2 || z >= 1 {
            // Only load a few chunks to keep things fast
            // TODO: let the user specify a specific part of the world to render
            return None;
        }
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
}

pub struct BlockStates {
    pub palette: Vec<Intern<Block>>,
    pub data: Box<[u16; 4096]>,
}

impl Default for BlockStates {
    fn default() -> Self {
        Self {
            palette: vec![Intern::new(Block {
                name: "minecraft:air".to_owned(),
            })],
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
    pub name: String,
}
