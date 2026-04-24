#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map};

#[contract]
pub struct ExamResults;

#[contractimpl]
impl ExamResults {
    /// Initialize the contract with an admin address
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().get::<_, bool>(&"initialized").is_some() {
            panic!("Already initialized");
        }
        env.storage().instance().set(&"initialized", &true);
        env.storage().instance().set(&"admin", &admin);
    }

    /// Publish an exam result (admin only)
    /// Stores student_id, score, and passed status
    pub fn publish_result(env: Env, admin: Address, student_id: Address, score: u32, passed: bool) {
        admin.require_auth();

        // Verify caller is admin
        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&"admin")
            .expect("Admin not set");

        if admin != stored_admin {
            panic!("Not authorized: caller is not admin");
        }

        // Store the result as a tuple (score, passed)
        let mut results: Map<Address, (u32, bool)> = env
            .storage()
            .instance()
            .get(&"results")
            .unwrap_or(Map::new(&env));

        results.set(student_id, (score, passed));
        env.storage().instance().set(&"results", &results);
    }

    /// Get a student's exam result (anyone can call)
    pub fn get_result(env: Env, student_id: Address) -> Option<(u32, bool)> {
        let results: Map<Address, (u32, bool)> = env
            .storage()
            .instance()
            .get(&"results")
            .unwrap_or(Map::new(&env));
        results.get(student_id)
    }
}
