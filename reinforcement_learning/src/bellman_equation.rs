use super::mdp::{State, Action};

// Bellman Equation
/*
 * Bellman equation define the fundamental learning flow using MDP
 *
 *   V(s) <- R(s') + γV(s')
 *
 *   This can be defined as recusive if it's calculated repeatedly on next state
 *   But most of the time V(s') the existing entry of a table or an nn
 *
 * The equation has variations based on the algorithm, but the basic structure is same
 */

// Below components are used to weight the value depending on the algorithm

// Policy (π)
// π(a | s)
fn _policy(_s: State, _a: Action) -> f32 {
    /*
     * Probability of taking action a from state s
     * Given or learnt based on the algorithm
     */
     0.5
}


// Value (V)
// V(s)
fn _value(_s: State) -> f32 {
    /*
     * Value of future reward of s, when following policy pi
     * Learnt
     */
    0.0
}

// Q_Value (Q)
// Q(s, a)
fn _q_value(_s: State, _a: Action) -> f32 {
    /*
     * Value of future reward of taking action a from state s, when following policy pi
     * Learned
     */
    0.5
}
