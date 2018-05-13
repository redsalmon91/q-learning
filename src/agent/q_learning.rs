use super::super::model;
use super::super::util::io_util;

pub fn q_learning(
    episode_count: usize,
    action_count: usize,
    learning_rate: f64,
    greedy_factor: f64,
    initial_state: u64,
    win_state: u64,
    lost_state: u64,
    apply_action_to_state_function: &Fn(u64, usize, &Vec<u64>) -> (u64, f64)) {

    let mut q_table = model::q_table::QTable::get_instance(action_count, learning_rate, greedy_factor);
    for episode in 0..episode_count {
        println!("============== Training Episode {} ===============", episode);

        let mut state = initial_state;
        let mut episode_history = vec![0u64];
        while state != win_state && state != lost_state {
            episode_history.push(state);
            let mut next_action = q_table.get_next_best_action(state, true);
            println!("Episode {} State {} Action {}", episode, state, io_util::map_action_to_unicode(next_action));

            let (future_state, rewards) = apply_action_to_state_function(state, next_action, &episode_history);
            q_table.update_rewards(state, future_state, next_action, rewards);
            state = future_state;
        }
    }

    println!("============== Simulating with Trained Knowledge ===============");
    let mut simulate_state = initial_state;
    while simulate_state != win_state && simulate_state != lost_state {
        let next_action = q_table.get_next_best_action(simulate_state, false);
        println!("State {} Action {}", simulate_state, io_util::map_action_to_unicode(next_action));
        let (future_state, _) = apply_action_to_state_function(simulate_state, next_action, &vec![0u64]);
        simulate_state = future_state;
    }

    let simulation_results_message = if simulate_state == win_state { "Reached the castle! " } else { "Reached the bomb!" };
    println!("Simulation Results: {}", simulation_results_message);
}