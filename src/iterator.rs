fn main() {
    // it lets you iterate over any collection like
    // vector, hashmap, et
    let vect = vec![1, 2, 3, 4, 5];

    // This takes an immutable refrence to the vector
    //     let vec_iter = vec.iter();

    // This is an mutable refrence to the vector
    //let vec_mut_iter = vec.iter_mut();
    //for val in vec_mut_iter {
    //    *val = *val + 1;
    //}
    // println!("{:?}", vec_mut_iter);

    //for val in vec_mut_iter {
    //    println!("Got {}", val);
    //}
    
    let sum: i32 = vect.iter().sum();

    println!("{}", sum);

    //while let Some(val) = vect.iter().next() {
    //   println!("{}", val);
    //}

    let vec2 = vec![1,2,3,4,5,6,7];
    for val in vec2.iter().map(|x| x+1){
        println!("{}", val);

    }
    let vec3 = vec![3,4,5,6,7,8,9,10,11,12,13,14,15];
    let resvec = task(vec3);

    println!("{:?}", resvec);

}


fn task(vec: Vec<i32>) -> Vec<i32> {
    // in this task we will filter the vec for odd values and double it and return the new vector
    

    // 1st Approach

    //let mut ret_vec = Vec::new();
    //for val in vec.iter().filter(|x| *x % 2 == 1).map(|x| x*2){
    //    ret_vec.push(val);
    //}
    
    // 2nd Approach

    let ret_vec = vec.iter().filter(|x| *x % 2 == 1).map(|x| x * 2).collect();
    
    return ret_vec;

}
