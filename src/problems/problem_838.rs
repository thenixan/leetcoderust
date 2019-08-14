pub struct Solution;

#[derive(Clone, Eq, PartialEq, Copy)]
enum Domino {
    Stand,
    Left,
    Right,
}

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let v: Vec<Domino> = dominoes
            .chars()
            .into_iter()
            .map(|c| match c {
                'L' => Domino::Left,
                'R' => Domino::Right,
                _ => Domino::Stand,
            })
            .collect();
        return Self::apply_tension(&v).iter().map(|d| match d {
            Domino::Left => 'L',
            Domino::Right => 'R',
            Domino::Stand => '.',
        }).collect();
    }

    fn apply_tension(v: &Vec<Domino>) -> Vec<Domino> {
        let mut result = vec![Domino::Stand; v.len()];

        let mut i: usize = 0;
        while i < v.len() {
            if v[i] == Domino::Stand {
                let start = i;
                while i < v.len() {
                    if v[i] != Domino::Stand {
                        break;
                    }
                    i += 1;
                }
                let end = i - 1;

                let l_tension = if start == 0 { None } else { v.get(start - 1) };
                let r_tension = v.get(end + 1);

                match (l_tension, r_tension) {
                    (Some(Domino::Left), Some(Domino::Left)) => {
                        for j in start..=end {
                            result[j] = Domino::Left;
                        }
                    }
                    (Some(Domino::Right), Some(Domino::Right)) => {
                        for j in start..=end {
                            result[j] = Domino::Right;
                        }
                    }
                    (Some(Domino::Right), Some(Domino::Left)) => {
                        let distance = end - start;
                        let (m_s, m_e) = if distance % 2 == 0 { (start + distance / 2 - 1, start + distance / 2 + 1) } else { (start + distance / 2, start + distance / 2 + 1) };
                        for j in start..=m_s {
                            result[j] = Domino::Right;
                        }
                        for j in m_e..=end {
                            result[j] = Domino::Left;
                        }
                    }
                    (None, Some(Domino::Left)) => {
                        for j in start..=end {
                            result[j] = Domino::Left;
                        }
                    }
                    (Some(Domino::Right), None) => {
                        for j in start..=end {
                            result[j] = Domino::Right;
                        }
                    }
                    _ => {}
                }
            } else {
                result[i] = v[i];
                i += 1;
            }
        }

        return result;
    }
}