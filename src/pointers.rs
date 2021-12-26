pub fn run() {
    // referance -  refarance a pointer inside memory
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("{:?}", (arr1, arr2));

    // vec is not premitive
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("{:?}", (&vec1, vec2));
}
