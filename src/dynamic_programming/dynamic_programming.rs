use std::collections::HashMap;

/*
 * Snake game
 *
 * Environment is a 3 x 3 blocked board
 * Snake head and fruit is randomly placed in a block
 * The goal of the head is to get the fruit
 *
 *     0    1;h   2
 *     3    4     5
 *     6    7     8;f
 */


#[derive(Debug, Eq, PartialEq, Hash)]
#[derive(Clone)]
struct State {
    head: i32,
    fruit: i32,
}

enum Action {
    Up,
    Down,
    Left,
    Right,
}

const GAMMA: f32 = 0.9;


// P(s_next | s, a)
fn world_model(s: &State, a: &Action, s_next: &State) -> f32 {
    // Fruit does not move
    if s.fruit != s_next.fruit {
        return 0.0;
    }

    let next_head = match a {
        Action::Up => {
            if s.head < 3 {
                s.head
            } else {
                s.head - 3
            }
        },

        Action::Down => {
            if s.head >= 6 {
                s.head
            } else {
                s.head + 3
            }
        },

        Action::Left => {
            if s.head % 3 == 0 {
                s.head
            } else {
                s.head - 1
            }
        },

        Action::Right => {
            if s.head % 3 == 2 {
                s.head
            } else {
                s.head + 1
            }
        },
    };

    if s_next.head == next_head {
        1.0
    } else {
        0.0
    }
}


// R(s, a, s_next)
fn reward(s: &State, a: &Action, s_next: &State) -> f32 {
    // Hit a wall
    if s.head == s_next.head {
        return -10.0;
    }

    // Reached the fruit
    if s_next.head == s_next.fruit {
        return 10.0;
    }

    // Normal movement
    -1.0
}


// π(a | s)
fn policy(s: &State, a: &Action) -> f32 {
    let head = s.head;

    match a {
        Action::Up => match head {
            0 | 1 | 2 => 0.0,
            _ => 0.25,
        },

        Action::Down => match head {
            6 | 7 | 8 => 0.0,
            _ => 0.25,
        },

        Action::Left => match head {
            0 | 3 | 6 => 0.0,
            _ => 0.25,
        },

        Action::Right => match head {
            2 | 5 | 8 => 0.0,
            _ => 0.25,
        },
    }
}


// V(s)
fn value_iteration(pi: &impl Fn(&State, &Action) -> f32, states: &Vec<State>, actions: &Vec<Action>, v_table: &mut HashMap<State, f32>) {
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
    let mut states = Vec::<State>::with_capacity(9 * 9);

    for h in 0..9 {
        for f in 0..9 {
            states.push(
                State {
                    head: h,
                    fruit: f,
                }
            );
        }
    }

    let actions = Vec::from([
        Action::Up,
        Action::Down,
        Action::Left,
        Action::Right,
    ]);

    let mut v_table: HashMap<State, f32> = states
        .iter()
        .cloned()
        .map(|s| (s, 0.0))
        .collect();

    // Learning
    for i in 0..100 {
        value_iteration(&policy, &states, &actions, &mut v_table);

        println!("\nIteration {}", i + 1);
        print_v_table(&v_table, 8);
    }
}


fn print_v_table(v_table: &HashMap<State, f32>, fruit: i32) {
    println!("Fruit = {}", fruit);

    for y in 0..3 {
        for x in 0..3 {
            let head = y * 3 + x;

            let state = State {
                head,
                fruit,
            };

            let value = v_table.get(&state).unwrap();

            print!("{:7.2}", value);
        }

        println!();
    }
}
