pub struct Portfolio {
    pub initial_investment: f64,    // The initial amount invested in the portfolio
    pub expected_yearly_return: f64, // The expected annual return rate of the portfolio (as a decimal, e.g., 0.07 for 7%)
    pub monthly_contributions: f64,  // Monthly contributions to the portfolio
    pub volatility: f64,              // The annual volatility of the portfolio (as a decimal, e.g., 0.15 for 15%)
    pub years: u32,                  // The number of years to simulate the portfolio's performance
    pub goal: f64,                      // The financial goal to be achieved by the end of the simulation period
    pub num_simulations: u32,           // The number of Monte Carlo simulations to run
}