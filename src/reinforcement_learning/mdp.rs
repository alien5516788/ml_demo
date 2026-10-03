// Markov Decision Process
/*
 * M = (S, A, P, R, gamma)
 * MDP only give the problem structure, not solution.
 * It basically says here are the all data and tools that help you to solve a problem.
 */


// 1. State
/*
 * All possible states within an environment
 */
#[derive(Clone, Copy)]
pub struct State;

pub const S: [State; 9] = [State, State, State, State, State, State, State, State, State];


// 2. Action
/*
 * All possible action can be taken within an environment
 */
#[derive(Clone, Copy)]
pub struct Action;

pub const A: [Action; 4] = [Action, Action, Action, Action];

// 3. World model
pub fn P(s: State, a: Action, s_next: State) -> f32 {
    /*
     * Probability of transitioning to next state
     * (s, a) = transition
     */
    0.0
}


// 4. Reward (there are variations)
pub fn R_1(s: State, a: Action, s_next: State) -> f32 {
    /*
     * The reward for getting to next state using this transition
     * transition = (s, a)
     */
    0.0
}

pub fn R_2(s: State, a: Action) -> f32 {
    /*
     * The Reward for using this transition
     * Doesn't care about the next state
     */
    0.0
}

pub fn R_3(s_next: State) -> f32 {
    /*
     * The reward for getting to next state
     * Doesn't care about the transition
     */
    0.0
}


// 5. Discount factor
/*
 * Reduce the reward of subsequest steps
 */
pub const gamma: f32 = 0.9;
