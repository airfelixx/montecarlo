//
//     This is a financial risk management tool that uses Monte Carlo simulations
//     to estimate the potential future value of an investment portfolio.
//
//     The tool allows users to input their investment portfolio details, including
//     the initial investment amount, expected annual return, and volatility. It then
//     simulates the portfolio's performance over a specified number of years and
//     generates a distribution of potential future values.
//
//     This program was developed by Miguel Correia, student at the
//     Instituto Superior Técnico, University of Lisbon, Portugal.
//
//     version 0.0.3
//     13/06/2025
//

mod simulation;
mod structs;

fn main() {
    //example portfolio configuration
    let portfolio = structs::Portfolio {  // Remove mut since we're not modifying it anymore
        initial_investment: 3340.0,          // Initial investment amount      
        expected_yearly_return: 0.1,         // Expected annual return (10%)
        monthly_contributions: 50.0,          // Monthly contributions to the portfolio
        volatility: 0.1,                      // Annual volatility (10%)
        years: 3,                             // Number of years to simulate
        goal: 5800.0,                         // Financial goal to be achieved
        num_simulations: 1000000,                 // Number of Monte Carlo simulations to run
    };
    simulation::run_monte_carlo_simulation(&portfolio);  // Run the Monte Carlo simulation with the configured portfolio
    println!("==========================");  // Print a separator line for better readability
    println!("Monte Carlo simulation completed.");  // Print a message indicating the completion of the simulation
    println!("Thank you for using the Monte Carlo simulation tool!");  // Print a thank you message to the user
    println!("==========================");  // Print another separator line
}
