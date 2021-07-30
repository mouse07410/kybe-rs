//! Params for crystals-kyber
//!
//! Public params for the PKE and KEM

/// Kyber public params collection
#[derive(Clone, Copy, Debug)]
pub struct KyberParams {
    /// Key entropy target
    pub n: usize,

    /// Lattice dimension
    pub k: usize,

    /// NTT ring size, n|(q-1)
    pub q: usize,

    /// Binomial distribution parameter
    pub eta: usize,

    /// Ciphertext parameters
    pub du: usize,
    pub dv: usize,

    /// Failure probability (2^-delta)
    pub delta: usize,

    /// Public key size (in bytes)
    /// This is 12*k*n/8+32 in CCAKEM
    pub pk_size: usize,

    /// Secret key size (in bytes)
    /// This is 24*k*n/8+96 in CCAKEM
    pub sk_size: usize,

    /// Ciphertext size (in bytes)
    /// This is du*k*n/8+dv*n/8 in CCAKEM
    pub ct_size: usize,
}

impl KyberParams {
    /// kyber-512 params
    pub const fn kyber512() -> Self {
        Self {
            n: 256,
            k: 2,
            q: 3329,
            eta: 2,
            du: 10,
            dv: 3,
            delta: 178,
            pk_size: 800,
            sk_size: 1632,
            ct_size: 738,
        }
    }

    /// kyber-768 params
    pub const fn kyber768() -> Self {
        Self {
            n: 256,
            k: 3,
            q: 3329,
            eta: 2,
            du: 10,
            dv: 4,
            delta: 164,
            pk_size: 1184,
            sk_size: 2400,
            ct_size: 1088,
        }
    }
}