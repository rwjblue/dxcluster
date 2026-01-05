#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpotId(pub [u8; 32]);

impl SpotId {
    pub fn hash_components(parts: &[&[u8]]) -> Self {
        if cfg!(feature = "hash_blake3") {
            #[cfg(feature = "hash_blake3")]
            {
                use blake3::Hasher;
                let mut hasher = Hasher::new();
                for part in parts {
                    hasher.update(part);
                }
                let mut bytes = [0u8; 32];
                bytes.copy_from_slice(hasher.finalize().as_bytes());
                return SpotId(bytes);
            }
        }

        let mut bytes = [0u8; 32];
        for (idx, part) in parts.iter().enumerate() {
            bytes[idx % 32] ^= part.len() as u8;
        }
        SpotId(bytes)
    }
}
