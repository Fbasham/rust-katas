struct PaginationHelper<T> {
    v: Vec<T>,
    k: usize,
}

impl<T> PaginationHelper<T> {
    fn new(collection: Vec<T>, items_per_page: usize) -> Self {
        Self {
            v: collection,
            k: items_per_page,
        }
    }

    fn item_count(&self) -> usize {
        self.v.len()
    }

    fn page_count(&self) -> usize {
        ((self.v.len() as f64) / self.k as f64).ceil() as usize
    }

    fn page_item_count(&self, i: usize) -> Option<usize> {
        if i >= self.page_count() {
            return None;
        }
        if i == self.page_count() - 1 {
            return if self.v.len() % self.k == 0 {
                Some(self.k)
            } else {
                Some(self.v.len() % self.k)
            };
        }
        Some(self.k)
    }

    fn page_index(&self, i: usize) -> Option<usize> {
        if self.v.len() == 0 || i >= self.v.len() {
            return None;
        }
        Some(i / self.k)
    }
}
