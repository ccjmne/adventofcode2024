use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{Arc, RwLock};

pub type Output = dyn Display + Send + Sync;

pub trait Solution {
    type Input;
    fn parse(source: String) -> Self::Input;
    fn part_i(input: &Self::Input) -> Box<Output>;
    fn part_ii(input: &Self::Input) -> Box<Output>;
}

#[derive(Clone)]
pub struct AnySolution {
    pub solve: Arc<dyn Fn(String) -> (Box<Output>, Box<Output>) + Send + Sync>,
}
impl AnySolution {
    pub fn new<S: Solution>() -> Self {
        let solve = Arc::new(move |source: String| {
            let input = S::parse(source);
            (S::part_i(&input), S::part_ii(&input))
        });
        AnySolution { solve }
    }
}

#[macro_export]
macro_rules! register {
    ($day:expr, $solution:ty) => {
        #[ctor]
        fn init() {
            register($day, AnySolution::new::<$solution>());
        }
    };
}

pub fn register(day: u8, solution: AnySolution) {
    SOLUTIONS.write().unwrap().insert(day, solution);
}

pub fn get(day: u8) -> Option<AnySolution> {
    SOLUTIONS.read().unwrap().get(&day).cloned()
}

static SOLUTIONS: Lazy<RwLock<HashMap<u8, AnySolution>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
