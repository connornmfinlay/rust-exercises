fn main() {
    let my_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Create a new array by concatenating slices before and after the elements to be removed
    let filtered_array1 = &my_array[..1];
    let filtered_array2 = &my_array[1..4];
    let filtered_array3 = &my_array[..5];
    let filtered_array4 = &my_array[5..];
    let filtered_array5 = &my_array[6..9];
    let filtered_array6 = &my_array[6..10];
    let random_index1 = 3;
    let random_index2= 7;
    let sliced_array = &my_array[random_index1..random_index2];
    println!("{:?}", filtered_array1);
    println!("{:?}", filtered_array2);
    println!("{:?}", filtered_array3);
    println!("{:?}", filtered_array4);
    println!("{:?}", filtered_array5);
    println!("{:?}", filtered_array6);
    println!("{:?}", sliced_array);
}
