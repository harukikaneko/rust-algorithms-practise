#[derive(Debug, Clone)]
pub struct Search {
    values: Vec<i32>,
}

impl Search {
    pub fn new(values: Vec<i32>) -> Self {
        Search { values }
    }

    pub fn linear_search(self, value: &i32) -> Option<i32> {
        for (i, v) in self.values.iter().enumerate() {
            if v == value {
                return Some(i as i32);
            }
        }
        None
    }

    pub fn binary_search(self, value: &i32) -> Option<i32> {
        let mut low = 0;
        let mut high = self.values.len() - 1;
        while low <= high {
            let mid = (low + high) / 2;
            match self.values[mid].cmp(value) {
                std::cmp::Ordering::Less => low = mid + 1,
                std::cmp::Ordering::Greater => high = mid - 1,
                std::cmp::Ordering::Equal => return Some(mid as i32),
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let search = Search::new(values);
        assert_eq!(search.clone().linear_search(&5), Some(4));
        assert_eq!(search.linear_search(&11), None);
    }

    #[test]
    fn test_binary_search() {
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let search = Search::new(values);
        assert_eq!(search.clone().binary_search(&5), Some(4));
        assert_eq!(search.binary_search(&11), None);
    }
}
