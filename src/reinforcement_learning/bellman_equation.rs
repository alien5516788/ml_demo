// Bellman Equation
/*
 * Bellman equation defines the mathematical relationship in MDP
 * The theoritical Bellman equation doesn't do any learning
 */
use super::mdp::{State, Action};
use super::mdp::{S, A, P, R_1 as R, gamma};


// Policy (there are variations)
fn deterministic_Pi(s: State, a: Action) -> f32 {
    /*
     * Given the state and action, output the probability of taking that action
     * So the action must be taken accordng to the probability
     * For deterministic policy each action have probability of 0 or 1 (always 1 action is taken)
     */
     1.0
}

fn stochastic_Pi(s: State, a: Action) -> f32 {
    /*
     * For stochastic policy each action have a probability
     * Given action or some other action may be taken based on proabability)
     */
    0.7
}


// Value
fn V(pi: &impl Fn(State, Action) -> f32, s: State) -> f32 {
    /*
     * In mdp the reward function gives the immediate reward
     * The value function gives the expected future reward (discounted) for that state when following policy function pi
     *     (there can be many policy functions)
     * For example for a certain state A, the reward function give 0 immediate reward,
     *     but the value function say it will lead to +10
     */
    11.2
}


// Bellman equation
/*
 * The mathematical definition for the V
 * In practical algorithms V is learned
 */
fn _V(pi: &impl Fn(State, Action) -> f32, s: State) -> f32 {
    let mut total = 0.0;

    for a in A {
        let action_probability = pi(s, a);

        for s_next in S {
            let transition_probability = P(s, a, s_next);

            let immediate_reward = R(s, a, s_next);
            let future_reward = gamma * _V(pi, s_next); // recursive

            // Probability weighted contribution of reward for every (a,s) pair
            total += action_probability * transition_probability * (immediate_reward + future_reward);
        }
    }

    total
}
