#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, symbol_short, String};

#[contracttype]
#[derive(Clone)]
pub struct Task {
    pub task_id: u64,
    pub title: String,
    pub is_completed: bool,
    pub timestamp: u64,
}

const TASK_COUNT: Symbol = symbol_short!("T_COUNT");

#[contracttype]
pub enum Taskbook {
    Task(u64),
}

#[contract]
pub struct TimeManagerContract;

#[contractimpl]
impl TimeManagerContract {
    pub fn create_task(env: Env, title: String) -> u64 {
        let mut task_count: u64 = env.storage().instance().get(&TASK_COUNT).unwrap_or(0);
        task_count += 1;

        let task = Task {
            task_id: task_count,
            title,
            is_completed: false,
            timestamp: env.ledger().timestamp(),
        };

        env.storage().instance().set(&Taskbook::Task(task.task_id), &task);
        env.storage().instance().set(&TASK_COUNT, &task_count);
        env.storage().instance().extend_ttl(5000, 5000);
        task.task_id
    }

    pub fn complete_task(env: Env, task_id: u64) {
        let mut task: Task = env.storage().instance().get(&Taskbook::Task(task_id)).unwrap();
        task.is_completed = true;
        env.storage().instance().set(&Taskbook::Task(task_id), &task);
    }

    pub fn view_task(env: Env, task_id: u64) -> Task {
        env.storage().instance().get(&Taskbook::Task(task_id)).unwrap_or(Task {
            task_id: 0,
            title: String::from_str(&env, "Not_Found"),
            is_completed: false,
            timestamp: 0,
        })
    }

    pub fn task_count(env: Env) -> u64 {
        env.storage().instance().get(&TASK_COUNT).unwrap_or(0)
    }
}