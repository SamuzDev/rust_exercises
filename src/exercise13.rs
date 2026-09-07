// PROBLEM: Item invoice. Add 16% VAT.
// If the gross price (price * quantity) is greater than $120,000, apply a 5% discount to the final net total.
// Returns the final price to pay.
pub fn calculate_invoice(unit_price: f64, quantity: u32) -> f64 {
    let gross_price = unit_price * quantity as f64;
    let net_total = gross_price * 1.16;

    if gross_price > 120_000.0 {
        net_total * 0.95
    } else {
        net_total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_invoice() {
        // Write your assertions based on your mathematical calculations
        let total = calculate_invoice(50_000.0, 3); // Gross: 150,000 (Applies discount)
        assert_eq!(total, 165_300.0);
    }
}