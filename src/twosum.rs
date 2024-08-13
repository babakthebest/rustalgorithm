use std::collections::HashMap;


pub fn two_sum(arr:&[i32],target:i32)->Vec<Vec<usize>>{
    println!("target is {}",target);
    let mut res:Vec<Vec<usize>>=Vec::new();
    let len: usize=arr.len();
    for i in 0..len{
        for j in (i+1)..len  {
            let sum= arr[i]+arr[j];
            if target==sum{
                res.push(vec![i,j]);
            }
            // println!("Sum of arr[{}] and arr[{}]: {} + {} = {}", i, j, arr[i], arr[j], sum);      
        }
    }
    return res;
}


pub fn two_sum_v2(arr:&[i32],target:i32)->Vec<Vec<usize>>{
        let mut res:Vec<Vec<usize>>=Vec::new();
    let mut  num_map: HashMap<i32, usize> =HashMap::new();
    for i in 0..arr.len(){
        let  complement=target - arr[i];

         if let Some(&complement_index) = num_map.get(&complement) {
             res.push(vec![complement_index, i]);
             
        }
        //  num_map[&arr[i]]=i
        num_map.insert(arr[i], i);
    }
    // println!("ARR IS {:?}",  res);
    return  res;
}