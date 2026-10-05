use std::collections::HashMap;

use crate::snake::{HEIGHT, WIDTH};
use crate::snake::mdp::{Action, GAMMA, State, actions, reward, states, world_model};


// Policy (defined)
fn policy(_s: &State, a: &Action) -> f32 {
    match a {
        Action::Up => 0.15,
        Action::Down => 0.15,
        Action::Left => 0.15,
        Action::Right => 0.15,
        Action::Eat => 0.4,
    }
}


// Value (learned)
fn v_table(states: &Vec<State>) -> HashMap<State, f32> {
    let table: HashMap<State, f32> = states
        .iter()
        .cloned()
        .map(|s| (s, 0.0))
        .collect();

    table
}


// Value is learned by continuously interacting with the world_model
fn iterative_policy_evaluation(pi: &impl Fn(&State, &Action) -> f32, states: &Vec<State>, actions: &Vec<Action>, v_table: &mut HashMap<State, f32>) {
    let mut new_table = HashMap::new();

    for s in states {
        // V_(k+1)(s)
        let mut total = 0.0;

        for a in actions {
            // π(a | s)
            let action_probability = pi(s, a);

            for s_next in states {
                // P(s_next | s, a)
                let transition_probability = world_model(s, a, s_next);

                if transition_probability == 0.0 {
                    continue;
                }

                // R(s, a, s_next)
                let immediate_reward = reward(s, a, s_next);

                // V_k(s_next)
                let future_value = v_table
                    .get(s_next)
                    .unwrap();

                total += action_probability
                    * transition_probability
                    * (immediate_reward + GAMMA * future_value);
            }
        }

        new_table.insert(s.clone(), total);
    }

    *v_table = new_table;
}



pub fn run() {
    let states = states();
    let actions = actions();
    let mut v_table: HashMap<State, f32> = v_table(&states);

    println!("\nInitial table");
    print_v_table(&v_table, (0, 0));

    // Learning
    for i in 0..50 {
        iterative_policy_evaluation(&policy, &states, &actions, &mut v_table);

        println!("\nIteration {}", i + 1);
        print_v_table(&v_table, (0, 0));
    }
}


fn print_v_table(v_table: &HashMap<State, f32>, fruit: (i32, i32)) {
    println!("Fruit: x = {} y = {}", fruit.0, fruit.1);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let state = State {
                head: (x, y),
                fruit,
            };

            let value = v_table.get(&state).unwrap();

            print!("{:7.2}", value);
        }

        println!();
    }
}
