mod common;
mod borrow_checker;
mod patterns;
mod layers;
mod strategies;
mod redis;
mod integration;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::init;
    
    #[test]
    fn run_all_tests() {
        // Initialize test environment
        init();
        
        // Tests are run automatically by cargo test
    }
} 