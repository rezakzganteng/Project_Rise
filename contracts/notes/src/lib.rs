#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Habit {
    pub id: u64,
    pub title: String,
    pub streak: u64,
    pub completed: bool,
}

const HABITS: Symbol = symbol_short!("HABITS");
const NEXT_ID: Symbol = symbol_short!("NEXTID");

#[contract]
pub struct HabitContract;

#[contractimpl]
impl HabitContract {
    // Tambah habit baru
    pub fn create_habit(env: Env, title: String) -> String {
        if title.len() == 0 {
            return String::from_str(&env, "Title cannot be empty");
        }

        let mut habits: Vec<Habit> = env
            .storage()
            .instance()
            .get(&HABITS)
            .unwrap_or(Vec::new(&env));

        let next_id: u64 = env.storage().instance().get(&NEXT_ID).unwrap_or(1);

        let habit = Habit {
            id: next_id,
            title,
            streak: 0,
            completed: false,
        };

        habits.push_back(habit);
        env.storage().instance().set(&HABITS, &habits);
        env.storage().instance().set(&NEXT_ID, &(next_id + 1));

        String::from_str(&env, "Habit created")
    }

    // Ambil semua habit
    pub fn get_habits(env: Env) -> Vec<Habit> {
        env.storage()
            .instance()
            .get(&HABITS)
            .unwrap_or(Vec::new(&env))
    }

    // Tandai habit selesai (menambah streak)
    pub fn complete_habit(env: Env, id: u64) -> String {
        let mut habits: Vec<Habit> = env
            .storage()
            .instance()
            .get(&HABITS)
            .unwrap_or(Vec::new(&env));

        for i in 0..habits.len() {
            let h = habits.get(i).unwrap();

            if h.id == id {
                let updated = Habit {
                    id: h.id,
                    title: h.title,
                    streak: h.streak + 1,
                    completed: true,
                };

                habits.set(i, updated);
                env.storage().instance().set(&HABITS, &habits);

                return String::from_str(&env, "Habit completed");
            }
        }

        String::from_str(&env, "Habit not found")
    }

    // Reset status harian (optional)
    pub fn reset_status(env: Env, id: u64) -> String {
        let mut habits: Vec<Habit> = env
            .storage()
            .instance()
            .get(&HABITS)
            .unwrap_or(Vec::new(&env));

        for i in 0..habits.len() {
            let h = habits.get(i).unwrap();

            if h.id == id {
                let updated = Habit {
                    id: h.id,
                    title: h.title,
                    streak: h.streak,
                    completed: false,
                };

                habits.set(i, updated);
                env.storage().instance().set(&HABITS, &habits);

                return String::from_str(&env, "Habit reset");
            }
        }

        String::from_str(&env, "Habit not found")
    }

    // Hapus habit
    pub fn delete_habit(env: Env, id: u64) -> String {
        let mut habits: Vec<Habit> = env
            .storage()
            .instance()
            .get(&HABITS)
            .unwrap_or(Vec::new(&env));

        for i in 0..habits.len() {
            if habits.get(i).unwrap().id == id {
                habits.remove(i);
                env.storage().instance().set(&HABITS, &habits);
                return String::from_str(&env, "Habit deleted");
            }
        }

        String::from_str(&env, "Habit not found")
    }
}

mod test;