use std::collections::HashMap;
const ADAPTER_RATING_LIST: &str = "71
183
111
89
92
142
25
101
52
86
18
22
70
2
135
163
34
143
153
35
144
24
23
94
100
102
17
57
76
182
134
38
7
103
66
31
11
121
77
113
128
82
99
148
137
41
32
48
131
60
127
138
73
28
10
84
180
63
125
53
176
165
114
145
152
72
107
167
59
164
78
126
118
136
83
79
58
14
106
69
51
39
157
42
177
173
93
141
3
33
13
19
45
154
95
170
54
181
6
151
1
112
96
115
85
108
166
160
40
122
12";

fn part1(joltage_adapter_list_str: &str) -> i32 {
    let mut joltage_adapters: Vec<i64> = joltage_adapter_list_str
        .split("\n")
        .map(|string| string.parse::<i64>().unwrap())
        .collect();
    joltage_adapters.insert(0, 0);
    joltage_adapters.sort_unstable();
    joltage_adapters.push(joltage_adapters.last().unwrap() + 3);

    let mut joltage_adapter_difference_count: HashMap<i64, i32> = HashMap::new();
    let mut adapter_iter = joltage_adapters.iter();
    let mut prev = adapter_iter.next().unwrap();

    for cur in adapter_iter {
        let difference = cur - prev;
        match joltage_adapter_difference_count.get_mut(&difference) {
            Some(diff_count) => {
                *diff_count = *diff_count + 1;
            }
            None => {
                joltage_adapter_difference_count.insert(difference, 1);
            }
        }

        prev = cur;
    }

    // println!("{:?}", joltage_adapters);
    // println!("{:?}", joltage_adapter_difference_count);
    return joltage_adapter_difference_count.get(&1).unwrap()
        * joltage_adapter_difference_count.get(&3).unwrap();
}

/// joltage adapters better be sorted in ascending order!!!!!
fn count_possible_adapter_arrangements(
    cache: &mut HashMap<i64, i64>,
    joltage_adapters: Vec<i64>,
    current_adapter_rating: i64,
    laptop_adapter_rating: i64,
) -> i64 {
    // let mut joltage_adapters: Vec<i64> = joltage_adapter_list_str.split("\n").map(|string| string.parse::<i64>().unwrap()).collect();
    // joltage_adapters.insert(0, 0);
    // joltage_adapters.sort_unstable();

    // let valid_adapters = joltage_adapters.iter().map(|x| *x).filter(|other_adapter| {
    //     (*other_adapter > current_adapter_rating) && 1 < (*other_adapter - current_adapter_rating) && (*other_adapter - current_adapter_rating)  < 3
    // }).collect::<Vec<i64>>();
    if let Some(val) = cache.get(&current_adapter_rating) {
        return *val;
    }
    if joltage_adapters.len() == 0 {
        let diff = laptop_adapter_rating - current_adapter_rating;
        if 1 <= diff && diff <= 3 {
            return 1;
        } else {
            return 0;
        }
    }

    let mut ret: i64 = 0;
    let only_valid_joltage_adapters = joltage_adapters.iter().filter(|j| {
        let diff = *j - current_adapter_rating;
        1 <= diff && diff <= 3
    });
    for (i, joltage_adapter) in only_valid_joltage_adapters.enumerate() {
        let childs_valid_adapters = joltage_adapters
            .iter()
            .skip(i + 1)
            .map(|i| *i)
            .collect::<Vec<i64>>();
        ret += count_possible_adapter_arrangements(
            cache,
            childs_valid_adapters,
            *joltage_adapter,
            laptop_adapter_rating,
        );
    }
    cache.insert(current_adapter_rating, ret);
    return ret;
}

pub fn day10_main() {
    let apple = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
    println!("{}", part1(apple));

    let mut joltage_adapters: Vec<i64> = ADAPTER_RATING_LIST
        .split("\n")
        .map(|string| string.parse::<i64>().unwrap())
        .collect();
    joltage_adapters.sort_unstable();
    let laptop_jolts = *(joltage_adapters.last().unwrap()) + 3;
    let mut cache: HashMap<i64, i64> = HashMap::new();
    println!(
        "{}",
        count_possible_adapter_arrangements(&mut cache, joltage_adapters, 0, laptop_jolts)
    );
}
