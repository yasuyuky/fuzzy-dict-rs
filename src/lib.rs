mod fuzzy_dict;
pub use fuzzy_dict::FuzzyDict;

#[cfg(test)]
mod tests {
    use FuzzyDict;
    #[test]
    fn monogram() {
        let mut dic = FuzzyDict::new(1);
        dic.insert("foo", 1);
        dic.insert("bar", 2);
        let result = dic.query("baz");
        assert_eq!(result, vec![(2, ("bar", &2))]);
    }

    #[test]
    fn digram() {
        let mut dic = FuzzyDict::new(2);
        dic.insert("foo", 1);
        dic.insert("bar", 2);
        let result = dic.query("baz");
        assert_eq!(result, vec![(1, ("bar", &2))]);
    }
}
