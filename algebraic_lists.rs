#[derive(Debug, PartialEq, Eq)]
enum Cons<T: Clone> {
  Cons(T, Box<Cons<T>>),
  Null
}

impl<T: Clone> Cons<T> {
  pub fn new(head: T, tail: Self) -> Self {
    Cons::Cons(head, Box::new(tail))
  }

  pub fn to_vec(&self) -> Vec<T> {
    match self {
      &Cons::Null => vec![],
      &Cons::Cons(ref head, ref tail) => {
        let mut head = vec![head.clone()];
        head.extend(tail.to_vec());
        head
      }
    }
  }
}

impl<T: Clone> Cons<T> {
  pub fn from_iter<I>(it: I) -> Self
    where I: IntoIterator<Item=T>
  {
      let mut v = vec![];
      let mut c = Cons::Null;
      for e in it {
          v.push(e);
      }
      for e in v.iter().cloned().rev() {   
          c = Cons::new(e,c);
      }
      return c
  }

  pub fn filter<F>(&self, f: F) -> Self
    where F: Fn(&T) -> bool
  {
    Cons::from_iter(self.to_vec().iter().cloned().filter(|e| f(e)))
  }

  pub fn map<F,S>(&self, f: F) -> Cons<S>
    where F: Fn(T) -> S, S: Clone
  {
    Cons::from_iter(self.to_vec().iter().cloned().map(|e| f(e)))
  }
}