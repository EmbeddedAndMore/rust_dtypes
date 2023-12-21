mod ring_mem;

fn plus(v1:&Vec<i32>, v2:&Vec<i32>)->Vec<i32>{
    println!("size: {}", v1.len());
    let mut new_vec:Vec<i32> = vec![0; v1.len()];
    for i in 0..v1.len(){
        new_vec[i] = v1[i]+v2[i];
    }
    return new_vec;
}


fn main() {
    println!("Hello, world!");
    let mut r_mem = ring_mem::RingMem::new(10);
    
    for i in 1..13{
        r_mem.add(i);
    }
    let res = plus(&vec![1,2,3,4], &vec![5,6,7,8]);
    
    // r_mem = RingMem::new(12);
    println!("{:?}", r_mem.buffer);
}
