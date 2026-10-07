// Markov Decision Process
/*
 * M = (S, A, P, R, gamma)
 * MDP only give the environment structure, not solution.
 */

// 1. States (S)
#[derive(Clone, Copy)]
pub struct State;

pub fn states() -> Vec<State> {
    /*
     * Set of all possible states
     */
    Vec::from([])
}

// 2. Actions (A)
#[derive(Clone, Copy)]
pub struct Action;

pub fn actions() -> Vec<Action> {
    /*
     * Set of all possible actions
     */
    return Vec::from([])
}

// 3. World model (P)
// Magically knows the dynamics of the environment (We don't care how the model knows it)
pub fn world_model(_s: State, _a: Action, _s_hat: State) -> f32 {
    /*
     * Probability of transitioning to from state s to an another state s_hat using action a
     */
    0.0
}


// 4. Reward (R)
// Magically knows the reward recieve from the environemnt (We don't care how it knows it)
pub fn reward(_s: State, _a: Action, _s_hat: State) -> f32 {
    /*
     * The immediate reward for transitioning from state s to an another state s_hat using action a
     * Can ignore s and a based on the algorithm
     */
    0.0
}


// 5. Discount factor (gamma)
/*
 * Reduce the reward of subsequest states
 */
pub const GAMMA: f32 = 0.9;
