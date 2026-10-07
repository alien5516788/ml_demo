use std::collections::HashMap;

use snake::{HEIGHT, WIDTH};
use snake::mdp::{State, states, Action, actions, world_model, reward, GAMMA};


// Value (V) - learnt
pub fn v_table(states: &Vec<State>) -> HashMap<State, f32> {
    let mut table: HashMap<State, f32> = HashMap::new();

    for s in states {
        table.insert(s.clone(), 0.0);
    }

    table
}

pub fn value(s: &State, v_table: &HashMap<State, f32>) -> f32 {
    v_table[s]
}


// Value iteration
/*
 * Value is learnt by continuously improving the value of each state
 * by choosing the action with the highest expected return.
 *
 *     V(s) <- max_a Σ_s_hat P(s_hat | s,a)
 *             * [R(s,a,s_hat) + γV(s_hat)]
 *
 *     or
 *
 *     Q(s,a) = Σ_s_hat P(s_hat | s,a)
 *              * [R(s,a,s_hat) + γV(s_hat)]
 *
 *     V(s) <- max_a Q(s,a)
 */
pub fn value_iteration(
    states: &Vec<State>,
    actions: &Vec<Action>,
    v_table: &mut HashMap<State, f32>
) {
    let mut new_table = HashMap::new();

    for s in states {
        // V(s)
        let mut best_value = f32::NEG_INFINITY; // max_a Q(s,a)

        // Σ_a / max_a
        for a in actions {
            // Q(s,a)
            let mut action_value = 0.0;

            // Σ_s_hat
            for s_hat in states {

                // P(s_hat | s,a)
                let transition_probability = world_model(s, a, s_hat);

                // Optional (optimization)
                if transition_probability == 0.0 {
                    continue;
                }

                // R(s,a,s_hat)
                let immediate_reward = reward(s, a, s_hat);

                // V(s_hat)
                let future_value = value(s_hat, v_table);

                // Add the s_hat contribution to Q(s,a)
                action_value += transition_probability
                    * (immediate_reward + (GAMMA * future_value));
            }

            // Keep the highest Q(s,a)
            if action_value > best_value {
                best_value = action_value; // max_a Q(s,a)
            }
        }

        // V(s) <- max_a Q(s,a)
        new_table.insert(s.clone(), best_value);
    }

    // Update v_table
    *v_table = new_table;
}


pub fn run() {
    let states = states();
    let actions = actions();
    let mut v_table = v_table(&states);

    println!("\nInitial table");
    print_v_table(&v_table, (3, 0));

    // Train
    for i in 0..50 {
        value_iteration(&states, &actions, &mut v_table);

        println!("\nIteration {}", i + 1);
        print_v_table(&v_table, (3, 0));
    }

    // Test
    todo!()
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
