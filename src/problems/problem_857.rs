#[derive(PartialEq, Clone)]
struct Worker {
    quality: f64,
    wage: f64,
}

impl Worker {
    pub fn new(quality: f64, wage: f64) -> Worker {
        return Worker {
            quality,
            wage,
        };
    }

    fn productivity(&self) -> f64 {
        return self.wage / self.quality;
    }
}

pub struct Solution {}

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let mut workers: Vec<Worker> = quality.iter().zip(wage.iter()).map(|(a, b)| { Worker::new(*a as f64, *b as f64) }).collect();
        workers.sort_by(|a, b| a.productivity().partial_cmp(&b.productivity()).unwrap());

        let mut min = 0f64;
        for i in 0..workers.len() {
            let result = Self::find_with_base(i, &workers, k);
            match result {
                Some(m) => {
                    if min == 0f64 || min > m {
                        min = m;
                    }
                }
                _ => {}
            }
        }
        return min;
    }

    fn find_with_base(base_item: usize, workers: &Vec<Worker>, k: i32) -> Option<f64> {
        let base_wage = workers[base_item].wage / workers[base_item].quality;

        let counted = k as usize - 1;
        let result = workers[base_item].wage;
        let mut prices: Vec<f64> = workers
            .iter()
            .enumerate()
            .filter_map(|(i, w)| {
                let price = base_wage * w.quality;
                if i != base_item && Self::more_or_eq(price, w.wage) {
                    Some(price)
                } else {
                    None
                }
            })
            .collect();
        prices.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        if prices.len() >= counted {
            return Some(Self::round_to_five(result + prices.iter().take(counted).sum::<f64>()));
        } else {
            return None;
        }
    }

    fn more_or_eq(a: f64, b: f64) -> bool {
        let ar = Self::round_to_five(a);
        let br = Self::round_to_five(b);
        return ar >= br;
    }

    fn round_to_five(a: f64) -> f64 {
        let factor = 10.0f64.powi(5);
        let r = (a * factor).round();
        return r / factor;
    }
}