use std::mem;
pub fn array_fun()
{
    let arr=[1,2,34,78,32,5];
    println!("array declaration :{:?}",arr);
    let arr_two=[0;6];
    println!("array initialize with same value: {:?}",arr_two);
    println!("size of the arr : {}",arr.len());

    println!("the arr  occupies : {} in bytes",mem::size_of_val(&arr));//because of 6 * byteof i32 that is 4

    let arr_2d =[[3;4];5];// 4 col 5 rows

    println!("the two dimensional array is of : {} by {} \n {:?} ",arr_2d.len(),arr_2d[0].len(),arr_2d);

    let arr_slice = &arr[2..];
    println!("array slicing from index 2, arr : {:?}  ",arr_slice);
}