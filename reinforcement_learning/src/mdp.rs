// Markov Decision Process
/*
 * M = (S, A, P, R, gamma)
 * MDP only give the environment structure, not solution.
 */

// 1. States (S)
#[derive(Clone, Copy)]
pub struct State;

impl State {
    pub fn states() -> Vec<State> {
        /*
         * Set of all possible states
         */
        Vec::from([])
    }
}


// 2. Actions (A)
#[derive(Clone, Copy)]
pub struct Action;

impl Action {
    pub fn actions() -> Vec<Action> {
        /*
         * Set of all possible actions
         */
        return Vec::from([])
    }
}


// 3. World model (P)
// P(s' | s, a)
pub fn world_model(_s: State, _a: Action, _s_next: State) -> f32 {
    /*
     * Probability of transitioning to from state s to next state s' using action a
     * Given (World model must be constructed separately)
     */
    0.0
}


// 4. Reward (R)
// R(s, a, s')
pub fn reward(_s: State, _a: Action, _s_next: State) -> f32 {
    /*
     * The immediate reward for transitioning from state s to next state s' using action a
     * Given or Recieved from the environment
     */
    0.0
}


// 5. Discount factor (γ)
/*
 * Reduce the reward of subsequest states
 */
pub const GAMMA: f32 = 0.9;
