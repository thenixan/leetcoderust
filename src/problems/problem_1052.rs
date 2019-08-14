pub struct Solution;

struct Minute {
    customers: i32,
    grumpy: bool,
}

impl Minute {
    fn grumpy(customers: i32) -> Self { Minute { customers, grumpy: true } }
    fn usual(customers: i32) -> Self { Minute { customers, grumpy: false } }
    fn lost_count(&self) -> i32 { if self.grumpy { self.customers } else { 0 } }
    fn worked_count(&self) -> i32 { if self.grumpy { 0 } else { self.customers } }
}

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
        let minutes: Vec<Minute> = (0..customers.len())
            .map(|i| {
                let count = customers[i];
                let is_grumpy = grumpy[i] == 1;
                if is_grumpy {
                    Minute::grumpy(count)
                } else {
                    Minute::usual(count)
                }
            })
            .collect();

        return Self::maximum_passed_clients(&minutes, x);
    }

    fn maximum_passed_clients(v: &Vec<Minute>, x: i32) -> i32 {
        let mut result = 0;
        let mut i: usize = 0;
        let mut m = 0;
        let mut max = 0;
        while i < v.len() {
            result += v[i].worked_count();
            m += v[i].lost_count();
            if i as i32 - x >= 0 {
                m -= v[i - x as usize].lost_count();
            }
            if m > max {
                max = m;
            }
            i += 1;
        }
        return result + max;
    }
}