mod mylib;
mod twosum;
use mylib::measure_performance;
use twosum::{two_sum, two_sum_v2};


fn main() {
    // let arr = [2, 8, 5, 4 ,3, 6];
     let arr: Vec<i32> = (1..=10000).collect();
    let target=19999;
let two_sum_v1 =two_sum(&arr,target);
let two_sum_v2=two_sum_v2(&arr,target);
measure_performance(|| two_sum(&arr, target), "two_sum_v1");
// println!("arr is :{:?}",arr);
println!("two_sum_v1 is :{:?}",two_sum_v1);
println!("two_sum_v2 is :{:?}",two_sum_v2);

}
