extern crate basichll as hll;

#[cfg(test)]
mod tests {
    use hll::*;

    #[test]
    fn a_hll_can_count_small_numbers () {
        let mut hll = HLL::new(0.0040625);

        hll.insert(&1);
        hll.insert(&2);
        hll.insert(&3);

        assert_eq!(hll.count().round(), 3.0);
    }

    #[test]
    fn a_hll_can_count_heterogenuous_items () {
        let mut hll = HLL::new(0.0040625);

        hll.insert(&1);
        hll.insert(&"foo");
        hll.insert(&2);
        hll.insert(&"bar");

        assert_eq!(hll.count().round(), 4.0);
    }
}
