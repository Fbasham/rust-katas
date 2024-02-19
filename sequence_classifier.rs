use itertools::Itertools;
use preloaded::Order;

fn sequence_classifier(a: &[i32]) -> Order {
    if a.iter().unique().count() == 1 {
        return Order::Constant;
    }
    if a.iter().zip(a.iter().skip(1)).all(|(i, j)| j > i) {
        return Order::Increasing;
    }
    if a.iter().zip(a.iter().skip(1)).all(|(i, j)| i > j) {
        return Order::Decreasing;
    }
    if a.iter().zip(a.iter().skip(1)).all(|(i, j)| j >= i) {
        return Order::NotDecreasing;
    }
    if a.iter().zip(a.iter().skip(1)).all(|(i, j)| i >= j) {
        return Order::NotIncreasing;
    }
    Order::Unordered
}
