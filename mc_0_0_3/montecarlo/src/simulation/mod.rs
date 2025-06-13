use crate::structs::Portfolio;
use rand_distr::{Normal, Distribution};

pub struct OneSimulationResult {
    final_value: f64,       // The final value of the portfolio after the simulation
}

pub struct SimulationsArray {
    simulations: Vec<OneSimulationResult>, // A vector containing the results of each individual simulation
}

pub struct MulSimulationResult {
    median_value: f64,                 // The median value of the portfolio across all simulations
    goal_probability: f64,              // The probability of reaching or exceeding the financial goal
    confidence_interval: (f64, f64),     // The confidence interval for the portfolio value at a specified confidence level
    confidence_level: f64,            // The confidence level for the confidence interval (e.g., 0.95 for 95%)
    //probability_distribution: Vec<f64>,  // The distribution of final portfolio values across all simulations, for next versions
}

pub fn simulate_yearly_return(expected: f64, volatility: f64) -> f64 {
    let normal = Normal::new(expected, volatility).unwrap();  // Create a normal distribution with the expected return and volatility
    let mut rng = rand::thread_rng();       // Create a random number generator
    normal.sample(&mut rng)         // Sample a yearly return from the normal distribution
}



pub fn simulate_portfolio(
    portfolio: &Portfolio,
) -> OneSimulationResult {
    let mut current_value = portfolio.initial_investment;    // Start with the initial investment amount

    for _ in 0..portfolio.years {             // Loop through each year of the simulation
        let yearly_return = simulate_yearly_return(      
            portfolio.expected_yearly_return,
            portfolio.volatility,
        );     // Simulate the yearly return based on expected return and volatility
        current_value *= 1.0 + yearly_return;    // Update current_value instead of portfolio.initial_investment
        current_value += portfolio.monthly_contributions * 12.0; // Annual contribution
    }

    OneSimulationResult { final_value: current_value }   // Return the final value of the portfolio after the simulation
}

pub fn run_simulations(portfolio: &Portfolio) -> SimulationsArray {
    let mut simulations = Vec::with_capacity(portfolio.num_simulations as usize);      // Create a vector to hold the results of each simulation

    for _ in 0..portfolio.num_simulations {     // Loop through the number of simulations to run
        let result = simulate_portfolio(portfolio);     // Simulate the portfolio for one iteration
        simulations.push(result);    // Store the result of the simulation in the vector
    }

    SimulationsArray { simulations }   // Return the vector of simulation results wrapped in a SimulationsArray struct
}

pub fn calculate_statistics(
    simulations: &SimulationsArray,
    portfolio: &Portfolio,
) -> MulSimulationResult {
    let mut final_values: Vec<f64> = simulations   
        .simulations
        .iter()
        .map(|s| s.final_value)
        .collect();     // Extract the final values from each simulation result

    final_values.sort_by(|a, b| a.partial_cmp(b).unwrap()); // Sort the final values in ascending order

    let median_index = final_values.len() / 2;       // Calculate the index for the median value
    let median_value = final_values[median_index];  // Get the median value from the sorted final values

    let goal_probability = final_values.iter().filter(|&&v| v >= portfolio.goal).count() as f64 / portfolio.num_simulations as f64;       // Calculate the probability of reaching or exceeding the financial goal

    let confidence_level = 0.90;     // Set the confidence level for the confidence interval (e.g., 90%)
    let lower_bound_index = (final_values.len() as f64 * (1.0 - confidence_level) / 2.0) as usize;   // Calculate the index for the lower bound of the confidence interval
    let upper_bound_index = (final_values.len() as f64 * (1.0 + confidence_level) / 2.0) as usize;     // Calculate the index for the upper bound of the confidence interval

    let confidence_interval = (
        final_values[lower_bound_index],
        final_values[upper_bound_index],
    );  // Create a tuple for the confidence interval using the values at the calculated indices

    MulSimulationResult {
        median_value,  
        goal_probability,
        confidence_interval,
        confidence_level,
        //probability_distribution: final_values,
    } // Return the results of the Monte Carlo simulation as a MulSimulationResult struct
}

pub fn print_results(
    result: &MulSimulationResult,
    portfolio: &Portfolio,
) {
    println!("Median Portfolio Value: {:.2}", result.median_value);       // Print the median value of the portfolio across all simulations
    println!(
        "Probability of reaching the goal of {:.2}: {:.2}%",
        portfolio.goal,
        result.goal_probability * 100.0
    );     // Print the probability of reaching or exceeding the financial goal
    println!(
        "Confidence Interval at {:.0}%: ({:.2}, {:.2})",
        result.confidence_level * 100.0,
        result.confidence_interval.0,
        result.confidence_interval.1
    );      // Print the confidence interval for the portfolio value at the specified confidence level

    
}

pub fn run_monte_carlo_simulation(portfolio: &Portfolio) {
    println!("==========================");  // Print a separator line for better readability
    println!("Running {} simulations...", portfolio.num_simulations);  // Print the number of simulations being run
    println!("==========================");  // Print another separator line
    let simulations = run_simulations(portfolio);  // Run the Monte Carlo simulations for the given portfolio
    let result = calculate_statistics(&simulations, portfolio);  // Calculate the statistics from the simulation results
    print_results(&result, portfolio);  // Print the results of the Monte Carlo simulation
}