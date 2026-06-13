#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String};

#[contract]
pub struct Contract;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Balance(String),
    Streak(String),
    Session(String, u64),
    Redeemed(String, String),
}

#[derive(Clone)]
#[contracttype]
pub struct FocusSession {
    pub minutes: u32,
    pub completed: bool,
    pub ai_score: u32,
    pub reward: i128,
}

#[contractimpl]
impl Contract {
    pub fn start_session(env: Env, user: String, session_id: u64, minutes: u32) {
        if minutes < 25 {
            panic!("Minimum focus session is 25 minutes");
        }

        let key = DataKey::Session(user.clone(), session_id);

        if env.storage().persistent().has(&key) {
            panic!("Session already exists");
        }

        let session = FocusSession {
            minutes,
            completed: false,
            ai_score: 0,
            reward: 0,
        };

        env.storage().persistent().set(&key, &session);

        env.events().publish((symbol_short!("start"), user), session_id);
    }

    pub fn complete_session(env: Env, user: String, session_id: u64, ai_score: u32) -> i128 {
        if ai_score > 100 {
            panic!("AI score must be from 0 to 100");
        }

        let session_key = DataKey::Session(user.clone(), session_id);

        let mut session: FocusSession = env
            .storage()
            .persistent()
            .get(&session_key)
            .expect("Session not found");

        if session.completed {
            panic!("Session already completed");
        }

        let mut reward: i128 = (session.minutes / 25) as i128;

        if ai_score >= 80 {
            reward += 1;
        }

        if session.minutes >= 50 {
            reward += 1;
        }

        session.completed = true;
        session.ai_score = ai_score;
        session.reward = reward;

        env.storage().persistent().set(&session_key, &session);

        let balance_key = DataKey::Balance(user.clone());
        let old_balance: i128 = env.storage().persistent().get(&balance_key).unwrap_or(0);
        let new_balance = old_balance + reward;
        env.storage().persistent().set(&balance_key, &new_balance);

        let streak_key = DataKey::Streak(user.clone());
        let old_streak: u32 = env.storage().persistent().get(&streak_key).unwrap_or(0);
        let new_streak = old_streak + 1;
        env.storage().persistent().set(&streak_key, &new_streak);

        env.events().publish((symbol_short!("done"), user), reward);

        reward
    }

    pub fn balance(env: Env, user: String) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(user))
            .unwrap_or(0)
    }

    pub fn streak(env: Env, user: String) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::Streak(user))
            .unwrap_or(0)
    }

    pub fn redeem(env: Env, user: String, item: String, cost: i128) -> i128 {
        if cost <= 0 {
            panic!("Cost must be greater than 0");
        }

        let balance_key = DataKey::Balance(user.clone());
        let old_balance: i128 = env.storage().persistent().get(&balance_key).unwrap_or(0);

        if old_balance < cost {
            panic!("Not enough FOCUS");
        }

        let new_balance = old_balance - cost;
        env.storage().persistent().set(&balance_key, &new_balance);

        let redeemed_key = DataKey::Redeemed(user.clone(), item.clone());
        env.storage().persistent().set(&redeemed_key, &true);

        env.events().publish((symbol_short!("redeem"), user), item);

        new_balance
    }

    pub fn award(env: Env, user: String, amount: i128) -> i128 {
        if amount <= 0 {
            panic!("Amount must be greater than 0");
        }

        let balance_key = DataKey::Balance(user.clone());
        let old_balance: i128 = env.storage().persistent().get(&balance_key).unwrap_or(0);
        let new_balance = old_balance + amount;

        env.storage().persistent().set(&balance_key, &new_balance);

        env.events().publish((symbol_short!("award"), user), amount);

        new_balance
    }
}