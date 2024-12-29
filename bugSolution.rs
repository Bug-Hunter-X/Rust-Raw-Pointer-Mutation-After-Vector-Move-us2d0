fn main() {
    let mut v = vec![1, 2, 3];
    let len = v.len();
    let mut v_copy = Vec::with_capacity(len);
    unsafe {
        let ptr = v.as_ptr();
        for i in 0..len {
            v_copy.push(*ptr.add(i));
        }
    }
    v_copy[0] = 4; 
    println!("{:?}", v_copy);
}