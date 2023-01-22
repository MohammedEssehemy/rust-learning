pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut min_price = prices[0];
    for &day_price in prices.iter() {
        if day_price < min_price {
            min_price = day_price;
        }

        if day_price - min_price > profit {
            profit = day_price - min_price;
        }
    }
    profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(prices), 5);
    }

    #[test]
    fn example2() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit(prices), 0);
    }
}
