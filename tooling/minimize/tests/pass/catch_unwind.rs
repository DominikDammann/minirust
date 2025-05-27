#![feature(core_intrinsics)]
#![allow(internal_features)]

extern crate intrinsics;
use intrinsics::*;


#[allow(unconditional_panic)]
/// This function always panics. It has the signature of a try function.
fn panic_try(_data_ptr: *mut u8) {
    let _a = 5 / 0;
    print(-1);
}

/// This function increases the value at the given data pointer by 1. It has the signature of a try function.
fn increase_data_try(data_ptr: *mut u8) {
    unsafe {
        *data_ptr = (*data_ptr) + 1;
    }
}

/// This function uses `catch_unwind`. It can be used to test nested catch structures. It has the signature of a try function.
fn nested_catch_try(data_ptr: *mut u8) {
    unsafe {
        core::intrinsics::catch_unwind(panic_try, data_ptr, print_data_catch);
    }
}

/// This function prints the value at the given data pointer. It has the signature of a catch function.
fn print_data_catch(data_ptr: *mut u8, _payload: *mut u8) {
    unsafe {
        print(*data_ptr);
    }
}

/// This function increases the value at the given data pointer by 1. It has the signature of a catch function.
fn increase_data_catch(data_ptr: *mut u8, _payload: *mut u8) {
    unsafe {
        *data_ptr = (*data_ptr) + 1;
    }
}

/// This function uses `catch_unwind`. It can be used to test nested catch structures. It has the signature of a catch function.
fn nested_catch_catch(data_ptr: *mut u8, _payload: *mut u8) {
    unsafe {
        core::intrinsics::catch_unwind(panic_try, data_ptr, print_data_catch);
    }
}

/// This function prints the value at the given data pointer.
/// It is used to check when the expression used as the data pointer gets evaluated.
fn evaluate_data_ptr(data_ptr: *mut u8) -> *mut u8 {
    unsafe {
        print(*data_ptr);
    }
    data_ptr
}


fn main() {
    let mut data: u8 = 5;
    let data_ptr = &mut data as *mut u8;
    
    print(0);

    // As `panic_try` panics, `print_data_try` will be executed.
    // This should print 5    
    let mut ret =  unsafe { core::intrinsics::catch_unwind(panic_try, data_ptr, print_data_catch) };
    assert!(ret == 1); 
    assert!(data == 5);

    print(0); 

    // `increase_data_ptr` does not panic, `print_data_try` will not be executed.
    ret = unsafe { core::intrinsics::catch_unwind(increase_data_try, data_ptr, print_data_catch) };
    assert!(ret == 0);
    assert!(data == 6); // data was increased by 1

    print(0);

    // The execution panics, however it gets caught in `inner_catch`. `inner_catch` prints 6
    ret = unsafe { core::intrinsics::catch_unwind(nested_catch_try, data_ptr, increase_data_catch) };
    assert!(ret == 0);
    assert!(data == 6);

    print(0);

    // `panic_try` panics. There is a panic in `inner_catch`, however it will be caught inside `inner_catch`
    // `inner_catch` prints 6
    ret = unsafe { core::intrinsics::catch_unwind(panic_try, data_ptr, nested_catch_catch) };
    assert!(ret == 1);
    assert!(data == 6);

    print(0);


    // make sure the data_ptr expression is only evaluated once
    ret = unsafe { core::intrinsics::catch_unwind(panic_try, evaluate_data_ptr(data_ptr), increase_data_catch) };
    assert!(ret == 1);
    assert!(data == 7);

    print(0);
}
