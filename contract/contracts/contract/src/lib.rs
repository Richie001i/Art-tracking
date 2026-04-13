#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env, String, Symbol,
};

// ── Storage Keys ──────────────────────────────────────────────────────────────

const ART_COUNT: Symbol = symbol_short!("ART_CNT");

// ── Data Types ────────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone)]
pub struct Artwork {
    pub id:       u64,
    pub title:    String,
    pub artist:   String,
    pub owner:    Address,
    pub ipfs_hash: String,  // IPFS CID for image / extended metadata
    pub year:     u32,
}

#[contracttype]
pub enum DataKey {
    Artwork(u64),
}

// ── Contract ──────────────────────────────────────────────────────────────────

#[contract]
pub struct ArtTrackContract;

#[contractimpl]
impl ArtTrackContract {

    /// Register a new artwork. Returns its unique ID.
    pub fn register(
        env:       Env,
        title:     String,
        artist:    String,
        owner:     Address,
        ipfs_hash: String,
        year:      u32,
    ) -> u64 {
        owner.require_auth();

        let id: u64 = env.storage().instance().get(&ART_COUNT).unwrap_or(0) + 1;

        let artwork = Artwork { id, title, artist, owner, ipfs_hash, year };

        env.storage().persistent().set(&DataKey::Artwork(id), &artwork);
        env.storage().instance().set(&ART_COUNT, &id);

        env.events().publish(
            (symbol_short!("register"), id),
            artwork.artist.clone(),
        );

        id
    }

    /// Transfer ownership to a new address.
    pub fn transfer(env: Env, id: u64, new_owner: Address) {
        let mut artwork: Artwork = env
            .storage()
            .persistent()
            .get(&DataKey::Artwork(id))
            .expect("artwork not found");

        artwork.owner.require_auth();
        new_owner.require_auth();

        artwork.owner = new_owner.clone();
        env.storage().persistent().set(&DataKey::Artwork(id), &artwork);

        env.events().publish(
            (symbol_short!("transfer"), id),
            new_owner,
        );
    }

    /// Fetch artwork details by ID.
    pub fn get(env: Env, id: u64) -> Artwork {
        env.storage()
            .persistent()
            .get(&DataKey::Artwork(id))
            .expect("artwork not found")
    }

    /// Total artworks registered.
    pub fn count(env: Env) -> u64 {
        env.storage().instance().get(&ART_COUNT).unwrap_or(0)
    }
}