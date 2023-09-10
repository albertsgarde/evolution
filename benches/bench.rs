use std::{iter::FilterMap, slice};

use criterion::{criterion_group, criterion_main, Criterion};
use evolution::EntityList;
use rand::{seq::SliceRandom, Fill, Rng};
use rand_pcg::Pcg64Mcg;

trait List {
    type T: 'static;

    fn new() -> Self;
    fn get(&self, index: usize) -> Option<&Self::T>;
    fn insert(&mut self, entity: Self::T) -> usize;
    fn remove(&mut self, index: usize);
}

impl<T> List for Vec<T>
where
    T: 'static,
{
    type T = T;

    fn new() -> Self {
        Vec::new()
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.get(index)
    }

    fn insert(&mut self, entity: T) -> usize {
        self.push(entity);
        self.len() - 1
    }

    fn remove(&mut self, index: usize) {
        self.remove(index);
    }
}

impl<T> List for EntityList<T>
where
    T: 'static,
{
    type T = T;

    fn new() -> Self {
        Self::new()
    }

    fn get(&self, index: usize) -> Option<&Self::T> {
        self.get(index)
    }

    fn insert(&mut self, entity: Self::T) -> usize {
        self.insert(entity)
    }

    fn remove(&mut self, index: usize) {
        self.remove(index)
    }
}

enum Operation<const SIZE: usize> {
    Insert([u8; SIZE]),
    Remove(usize),
}

fn init<L: List<T = [u8; SIZE]>, const SIZE: usize>(len: usize) -> L
where
    [u8; SIZE]: Fill,
{
    let mut rng = Pcg64Mcg::new(0);
    let mut list = L::new();
    let data = (0..2 * len).map(|_| {
        let mut item = [0u8; SIZE];
        rng.fill(&mut item);
        item
    });
    let mut operations: Vec<_> = data
        .map(Operation::Insert)
        .chain((0..len).map(|_| Operation::Remove(rng.gen())))
        .collect();
    operations[len..].shuffle(&mut rng);
    for operation in operations {
        match operation {
            Operation::Insert(item) => {
                list.insert(item);
            }
            Operation::Remove(index) => {
                list.remove(index);
            }
        }
    }

    list
}

fn bench_list<L: List<T = [u8; SIZE]>, const SIZE: usize>(c: &mut Criterion, len: usize)
where
    [u8; SIZE]: Fill,
{
    let list = init::<L, SIZE>(len);
    let mut rng = Pcg64Mcg::new(0);
    let mut indices: Vec<_> = (0..len).map(|_| rng.gen_range(0..len)).collect();
    indices.shuffle(&mut rng);

    c.bench_function(&format!("vec rem {size} {len}"), |b| {
        b.iter(|| for _ in list.iter() {})
    });

    c.bench_function(&format!("list::get::<{}>::{}", SIZE, len), |b| {
        b.iter(|| {
            for &index in &indices {
                list.get(index);
            }
        })
    });
}

pub fn bench(c: &mut Criterion) {
    for n in [20, 30, 40] {
        bench_fib(c, n);
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
