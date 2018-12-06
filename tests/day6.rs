extern crate day6;
extern crate utils;

use utils::Day;

#[test]
fn day6_a() {
    let solver = day6::Day6::new(String::from("1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"));

    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "17");
}

#[test]
fn day6_b() {
    // test contains friend's input, mine happened to be the glitched one and I wanted to verify if the solver works
    let solver = day6::Day6::new(String::from("59, 110
127, 249
42, 290
90, 326
108, 60
98, 168
358, 207
114, 146
242, 170
281, 43
233, 295
213, 113
260, 334
287, 260
283, 227
328, 235
96, 259
232, 177
198, 216
52, 115
95, 258
173, 191
156, 167
179, 135
235, 235
164, 199
248, 180
165, 273
160, 297
102, 96
346, 249
176, 263
140, 101
324, 254
72, 211
126, 337
356, 272
342, 65
171, 295
93, 192
47, 200
329, 239
60, 282
246, 185
225, 324
114, 329
134, 167
212, 104
338, 332
293, 94
"));

    let ans = solver.get_part_b_result();

    assert_eq!(&ans, "46554");
}
