use std::collections::HashMap;

use snake::{HEIGHT, WIDTH};
use snake::mdp::{State, states, Action, actions, world_model, reward, GAMMA};


// Policy (π) - defined
fn policy(_s: &State, a: &Action) -> f32 {
    match a {
        Action::Up => 0.15,
        Action::Down => 0.15,
        Action::Left => 0.15,
        Action::Right => 0.15,
        Action::Eat => 0.4,
    }
}


// Value (V) - learned
fn v_table(states: &Vec<State>) -> HashMap<State, f32> {
    let table: HashMap<State, f32> = states
        .iter()
        .cloned()
        .map(|s| (s, 0.0))
        .collect();

    table
}


/*
 * Value is learned by continuously interacting with the world_model
 *
 *     V(s) <- Σ_a Σ_s_hat π(a | s) * P(s_hat | s,a) * [R(s,a,s_hat) + γV(s_hat)]
 */
fn policy_evaluation(pi: &impl Fn(&State, &Action) -> f32, states: &Vec<State>, actions: &Vec<Action>, v_table: &mut HashMap<State, f32>) {
    let mut new_table = HashMap::new();

    for s in states {
        // V(s)
        let mut total = 0.0;

        for a in actions {
            // π(a | s)
            let action_probability = pi(s, a);

            for s_hat in states {
                // P(s_hat | s, a)
                let transition_probability = world_model(s, a, s_hat);

                // Skip the states that have 0 contribution (optional)
                // Not a part of original bellman equation
                if transition_probability == 0.0 {
                    continue;
                }

                // R(s, a, s_hat)
                let immediate_reward = reward(s, a, s_hat);

                // V(s_hat)
                let future_value = v_table
                    .get(s_hat)
                    .unwrap();

                // V(s) <- π(a | s) * P(s' | s,a) * [R(s,a,s') + γV(s')]
                total += action_probability
                    * transition_probability
                    * (immediate_reward + GAMMA * future_value);
            }
        }

        // Add state entry
        new_table.insert(s.clone(), total);
    }

    // Update v_table
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
        policy_evaluation(&policy, &states, &actions, &mut v_table);

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
