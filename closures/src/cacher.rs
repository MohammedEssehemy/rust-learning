
use std::{hash::Hash, collections::HashMap, thread, time::Duration};

pub struct Cacher<'a, Args, V> where Args: Eq + Hash + Copy, V: Copy
{
    calculation: &'a dyn Fn(Args) -> V,
    value: HashMap<Args, V>,
}

impl<'a, Args, V> Cacher<'a, Args, V>
where Args: Eq + Hash + Copy, V: Copy {
    pub fn new(calculation: &'a dyn Fn(Args) -> V) -> Self {
        Self {
            calculation,
            value: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: Args) -> V {
        match self.value.get(&arg) {
            None => {
                let res = (self.calculation)(arg);
                self.value.insert(arg, res);
                res
            }
            Some(res) => *res,
        }
    }
}


pub fn try_cacher() {
    println!("--------------------");
    println!("try_cacher");
    println!("--------------------");



    let calculation = |num: u32| {
        println!("calculating slowly u32...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let mut expensive_result = Cacher::new(&calculation);
    println!("expensive_result: Today, do {} pushups!", expensive_result.value(4));
    println!("expensive_result: Next, do {} situps!", expensive_result.value(5));
    println!("expensive_result: another, do {} pushups!", expensive_result.value(4));
    
    let calculation2 = |num: u16| {
        println!("calculating slowly u16...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let mut expensive_result2 = Cacher::new(&calculation2);
    println!("expensive_result2: Today, do {} pushups!", expensive_result2.value(4));
    println!("expensive_result2: Next, do {} situps!", expensive_result2.value(5));
    println!("expensive_result2: another, do {} pushups!", expensive_result2.value(4));
}