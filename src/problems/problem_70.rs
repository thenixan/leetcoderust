pub struct Solution {}

impl Solution {
    /**
    2:
    11
    2

    3:
    111
    21
    12

    4:
    1111
    211
    121
    112
    22


    5:
    11111
    2111
    1211
    1121
    1112
    221
    212
    122

    6:
    111111
    21111
    12111
    11211
    11121
    11112
    2211
    2121
    2112
    1221
    1212
    1122
    222

    7:
    1111111
    211111
    121111
    112111
    111211
    111121
    111112

    22111
    12211
    11221
    11122

    21211
    21121
    21112
    12121
    12112
    11212
    2221
    2212
    2122
    1222

    8:
    11111111
    2111111
    1211111
    1121111
    1112111
    1111211
    1111121
    1111112

    221111
    122111
    112211
    111221
    111122

    212111
    211211
    211121
    211112
    121211
    121121
    121112
    112121
    112112
    111212

    22211
    12221
    11222

    22121
    22112
    21221
    21212
    21122
    12212
    12122
    2222
    **/
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        } else if n == 2 {
            return 2;
        } else if n == 3 {
            return 3;
        } else {
            let mut this = 2;
            let mut prev = 1;
            for i in 3..=n {
                let n = this + prev;
                prev = this;
                this = n;
            }
            return this;
        }
    }
}