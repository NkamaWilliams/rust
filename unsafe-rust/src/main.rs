use std::slice;

unsafe extern "C" {
    unsafe fn abs(input: i32) -> i32;
}

static mut COUNTER: i32 = 0;

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("{}" ,*r2);
        println!("{}" ,*r1);
        println!("The absolute value of -8 according to C is: {}", abs(-8));
    }

    let mut arr = [1, 2, 3, 4, 5, 6];
    let (upper, lower) = my_split_at_mut(&mut arr, 3);
    println!("{:?}",upper);
    println!("{:?}",lower);

    unsafe_loop();
    unsafe {
        add_to_count(17 as i32);
        println!("COUNTER is currently {}", COUNTER);
    }
}

fn my_split_at_mut(arr:&mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = arr.len();
    let ptr = arr.as_mut_ptr();

    assert!(mid < len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut( ptr.add(mid), len - mid)
        )
    }
}

unsafe fn add_to_count(num: i32) {
    unsafe {
        COUNTER += num;
    }
}

fn unsafe_loop(){
    let mut address = 0x22345usize;
    let mut ptr = &mut address as *mut usize;
    while ptr.is_null() {
        unsafe {
            ptr.add(1);
        }
    }
    unsafe {
        println!("And the first unsafe address is {:#?} with the value {}", ptr, *ptr);
    }
}