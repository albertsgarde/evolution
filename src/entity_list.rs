use std::collections::VecDeque;

pub struct EntityList<T> {
    entities: Vec<Option<T>>,
    free: VecDeque<usize>,
}

impl<T> EntityList<T> {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            free: VecDeque::new(),
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.entities.get(index).and_then(|e| e.as_ref())
    }

    pub fn insert(&mut self, entity: T) -> usize {
        if let Some(index) = self.free.pop_front() {
            self.entities[index] = Some(entity);
            index
        } else {
            self.entities.push(Some(entity));
            self.entities.len() - 1
        }
    }

    pub fn remove(&mut self, index: usize) {
        self.entities[index] = None;
        self.free.push_back(index);
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.entities.iter().filter_map(|e| e.as_ref())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.entities.iter_mut().filter_map(|e| e.as_mut())
    }
}

impl<T> Default for EntityList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for EntityList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let entities = iter.into_iter().map(Some).collect();
        Self {
            entities,
            free: VecDeque::new(),
        }
    }
}
