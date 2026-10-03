// Bellman Equation
/*
 * Bellman equations define mathematical relationships
 * between rewards and value functions in an MDP.
 *
 * The equations themselves do not perform learning.
 * Algorithms such as policy evaluation and value iteration
 * repeatedly apply these relationships to compute value estimates.
 */
use super::mdp::{State, Action};
use super::mdp::{states, actions, world_model, reward, GAMMA};


// Policy (PI)
// Not magic
// We must decide the approach to select an action with a good balance
fn policy(_s: State, _a: Action) -> f32 {
    /*
     * Probability of taking action a from state s
     */
     0.5
}


// Value (V)
/*
 * Value of future reward of s, when following policy pi
 * Value is expected/computed
 * Must be learned
 */
fn _value(s: State) -> f32 {
    let mut total = 0.0;

    for a in actions() {
        let action_probability = policy(s, a);

        for s_hat in states() {
            let transition_probability = world_model(s, a, s_hat);
            let immediate_reward = reward(s, a, s_hat);
            let future_reward = GAMMA * _value(s_hat); // recursive

            // Probability weighted contribution of reward for every (a,s) pair
            total += action_probability * transition_probability * (immediate_reward + future_reward);
        }
    }

    total
}

// Q_Value (Q)
/*
 * Value of future reward of s given that the action a is taken
 * Must be learned
 */
fn _q_value(s: State, a: Action) -> f32 {
    let mut total = 0.0;

    for s_hat in states() {
        let transition_probability = world_model(s, a, s_hat);
        let immediate_reward = reward(s, a, s_hat);

        let mut future_reward = 0.0;

        for a_hat in actions() {
            let action_probability = policy(s_hat, a_hat);

            let next_q_value = _q_value(s_hat, a_hat);

            future_reward += action_probability * next_q_value;
        }

        total += transition_probability * (immediate_reward + GAMMA * future_reward);
    }

    total
}
