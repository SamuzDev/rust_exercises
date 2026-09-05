// PROBLEM: Item invoice. Add 16% VAT.
// If the gross price (price * quantity) is greater than $120,000, apply a 5% discount to the final net total.
// Returns the final price to pay.
pub fn calculate_invoice(unit_price: f64, quantity: u32) -> f64 {
    // TODO: Your code here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_invoice() {
        // Write your assertions based on your mathematical calculations
        let total = calculate_invoice(50000.0, 3); // Gross: 150,000 (Applies discount)
        assert!(total > 0.0);
    }
}