use std::collections::HashMap;

pub struct FuzzyDict<T> {
    n: usize,
    ngram_maps: HashMap<String, HashMap<usize, usize>>,
    id2key: Vec<String>,
    key2value: HashMap<String, T>,
}

impl<T> FuzzyDict<T> {
    pub fn new(n: usize) -> Self {
        Self { n,
               ngram_maps: HashMap::new(),
               id2key: Vec::new(),
               key2value: HashMap::new(), }
    }

    pub fn insert(&mut self, key: &str, value: T) {
        if !self.key2value.contains_key(key) {
            let id = self.id2key.len();
            self.id2key.push(String::from(key));
            for ngram in self.get_ngrams(key) {
                let ngram_map = self.ngram_maps.entry(ngram).or_insert(HashMap::new());
                *ngram_map.entry(id).or_insert(0) += 1;
            }
        }
        self.key2value.insert(String::from(key), value);
    }

    pub fn get(&self, key: &str) -> Option<&T> { self.key2value.get(key) }

    pub fn query(&self, key: &str) -> Vec<(usize, (&str, &T))> {
        let mut counter: HashMap<usize, usize> = HashMap::new();
        for ngram in self.get_ngrams(key) {
            self.ngram_maps.get(&ngram).and_then(|ngram_map| {
                                                     for (i, c) in ngram_map {
                                                         *counter.entry(*i).or_insert(0) += c;
                                                     }
                                                     Some(())
                                                 });
        }
        let mut result = counter.into_iter()
                                .map(|(i, c)| (c, &self.id2key[i]))
                                .collect::<Vec<(usize, &String)>>();
        result.sort_by(|(a, _), (b, _)| b.cmp(a));
        result.into_iter()
              .map(|(c, k)| (c, (k.as_str(), &self.key2value[k])))
              .collect()
    }

    pub fn get_or_search(&self, key: &str) -> Vec<(String, &T)> {
        match self.get(key) {
            Some(v) => vec![(String::from(key), v)],
            None => self.query(key).into_iter()
                        .map(|(_, (k, v))| (String::from(k), v))
                        .collect(),
        }
    }

    fn get_ngrams(&self, key: &str) -> Vec<String> {
        if key.len() < self.n {
            vec![String::from(key)]
        } else {
            let mut ngrams = Vec::new();
            for i in 0..key.len() - self.n {
                ngrams.push(String::from(&key[i..i + self.n]))
            }
            ngrams
        }
    }
}
