#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Vec, Symbol};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Note {
    id: u64,
    title: String,
    content: String
}

const NOTE_DATA: Symbol = symbol_short!("NOTE_DATA");

#[contract]   
pub struct NotesContract;

// This is a sample contract. Replace this placeholder with your own contract logic.
// A corresponding test example is available in test.rs.
//
// For comprehensive examples, visit <https://github.com/stellar/soroban-examples>.
// The repository includes use cases for the Stellar ecosystem, such as data storage on
// the blockchain, token swaps, liquidity pools, and more.
//
// Refer to the official documentation:
// <https://developers.stellar.org/docs/build/smart-contracts/overview>.
#[contractimpl]
impl NotesContract {
    pub fn get_notes(env: Env) -> Vec<Note> {
        // 1. ambil data notes dari storage
        return env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
    }

    // Fungsi untuk membuat note baru
    pub fn create_note(env: Env, title: String, content: String) -> String {
        // 1. ambil data notes dari storage
        let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
        
        // 2. Buat object note baru
        let note = Note {
            id: env.prng().gen::<u64>(),
            title: title,
            content: content,
        };
        
        // 3. tambahkan note baru ke notes lama
        notes.push_back(note);
        
        // 4. simpan notes ke storage
        env.storage().instance().set(&NOTE_DATA, &notes);
        
        return String::from_str(&env, "Notes berhasil ditambahkan");
    }

    // Fungsi untuk menghapus notes berdasarkan id
    pub fn delete_note(env: Env, id: u64) -> String {
        // 1. ambil data notes dari storage 
        let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));

        // 2. cari index note yang akan dihapus menggunakan perulangan
        for i in 0..notes.len() {
            if notes.get(i).unwrap().id == id {
                notes.remove(i);

                env.storage().instance().set(&NOTE_DATA, &notes);
                return String::from_str(&env, "Berhasil hapus notes");
            }
        }

        return String::from_str(&env, "Notes tidak ditemukan")
    }
}

mod test;