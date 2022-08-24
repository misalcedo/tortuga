use std::cmp::Ordering;
use std::mem;

#[derive(Clone, Debug)]
pub struct NonEmptyStack<Item> {
    top: Item,
    rest: Vec<Item>,
}

impl<I> NonEmptyStack<I> {
    pub fn len(&self) -> usize {
        self.rest.len() + 1
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn top(&self) -> &I {
        &self.top
    }

    pub fn top_mut(&mut self) -> &mut I {
        &mut self.top
    }

    pub fn push(&mut self, top: I) {
        self.rest.push(mem::replace(&mut self.top, top));
    }

    pub fn pop(&mut self) -> Option<I> {
        self.rest.pop()
    }
}

impl<I: Default> NonEmptyStack<I> {
    pub fn try_pop(&mut self) -> Result<I, I> {
        self.rest
            .pop()
            .ok_or_else(|| mem::replace(&mut self.top, Default::default()))
    }
}

impl<I> From<I> for NonEmptyStack<I> {
    fn from(top: I) -> Self {
        NonEmptyStack { top, rest: vec![] }
    }
}

impl<I: Default> Default for NonEmptyStack<I> {
    fn default() -> Self {
        NonEmptyStack {
            top: Default::default(),
            rest: Default::default(),
        }
    }
}

impl<I: PartialEq> PartialEq for NonEmptyStack<I> {
    fn eq(&self, other: &Self) -> bool {
        self.top.eq(&other.top) && self.rest.eq(&other.rest)
    }

    fn ne(&self, other: &Self) -> bool {
        self.top.ne(&other.top) || self.rest.ne(&other.rest)
    }
}

impl<I: Eq> Eq for NonEmptyStack<I> {}

impl<I: PartialOrd> PartialOrd for NonEmptyStack<I> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.top
            .partial_cmp(&other.top)
            .or_else(|| self.rest.partial_cmp(&other.rest))
    }

    fn lt(&self, other: &Self) -> bool {
        self.top.lt(&other.top) || self.rest.lt(&other.rest)
    }

    fn le(&self, other: &Self) -> bool {
        self.top.le(&other.top) || self.rest.le(&other.rest)
    }

    fn gt(&self, other: &Self) -> bool {
        self.top.gt(&other.top) || self.rest.gt(&other.rest)
    }

    fn ge(&self, other: &Self) -> bool {
        self.top.ge(&other.top) || self.rest.ge(&other.rest)
    }
}

impl<I: Ord> Ord for NonEmptyStack<I> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.top.cmp(&other.top) {
            Ordering::Equal => self.rest.cmp(&other.rest),
            ordering => ordering,
        }
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        match self.cmp(&other) {
            Ordering::Less => other,
            Ordering::Equal => self,
            Ordering::Greater => self,
        }
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        match self.cmp(&other) {
            Ordering::Less => self,
            Ordering::Equal => self,
            Ordering::Greater => other,
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        if self.cmp(&min) == Ordering::Less {
            min
        } else if self.cmp(&max) == Ordering::Greater {
            max
        } else {
            self
        }
    }
}
