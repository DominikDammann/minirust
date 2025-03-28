use crate::*;

#[test]
fn reach_catch_fn() {
    let mut p = ProgramBuilder::new();

    let try_fn = {
        let mut f = p.declare_function();
        let cleanup = f.cleanup(|f| f.resume_unwind());
        f.start_unwind(cleanup);
        p.finish_function(f)
    };

    let catch_fn = {
        let mut f = p.declare_function();
        f.print(const_int(42));
        f.return_();
        p.finish_function(f)
    };

    let main_fn = {
        let mut f = p.declare_function();
        let x = f.declare_local::<i32>();
        f.storage_live(x);
        f.catch_unwind(fn_ptr(try_fn), unit_place(), fn_ptr(catch_fn), x);
        f.exit();
        p.finish_function(f)
    };

    let p = p.finish_program(main_fn);
    dump_program(p);
    assert_eq!(get_stdout::<BasicMem>(p).unwrap(), &["42"]);
}

// Test the return value of catch_unwind in the case where try-func does not unwind.
#[test]
fn return_val_no_unwind() {
    let mut p = ProgramBuilder::new();

    let try_fn = {
        let mut f = p.declare_function();
        f.return_();
        p.finish_function(f)
    };

    let catch_fn = {
        let mut f = p.declare_function();
        f.unreachable();
        p.finish_function(f)
    };

    let main_fn = {
        let mut f = p.declare_function();
        let x = f.declare_local::<i32>();
        f.storage_live(x);
        f.catch_unwind(fn_ptr(try_fn), unit_place(), fn_ptr(catch_fn), x);
        f.assume(eq(load(x), const_int(0)));
        f.exit();
        p.finish_function(f)
    };

    let p = p.finish_program(main_fn);
    dump_program(p);
    assert_stop::<BasicMem>(p);
}

// Test the return value of catch_unwind in the case where try-func unwinds.
#[test]
fn return_val_unwind() {
    let mut p = ProgramBuilder::new();

    let try_fn = {
        let mut f = p.declare_function();
        let cleanup = f.cleanup_resume();
        f.start_unwind(cleanup);
        p.finish_function(f)
    };

    let catch_fn = {
        let mut f = p.declare_function();
        f.return_();
        p.finish_function(f)
    };

    let main_fn = {
        let mut f = p.declare_function();
        let x = f.declare_local::<i32>();
        f.storage_live(x);
        f.catch_unwind(fn_ptr(try_fn), unit_place(), fn_ptr(catch_fn), x);
        f.assume(eq(load(x), const_int(1)));
        f.exit();
        p.finish_function(f)
    };

    let p = p.finish_program(main_fn);
    dump_program(p);
    assert_stop::<BasicMem>(p);
}

#[test]
fn test_data_ptr() {
    let mut p = ProgramBuilder::new();

    let try_fn = {
        let mut f = p.declare_function();
        let x = f.declare_ret::<i32>();
        f.assign(x, const_int(42));
        f.return_();
        p.finish_function(f)
    };

    let catch_fn = {
        let mut f = p.declare_function();
        f.unreachable();
        p.finish_function(f)
    };

    let main_fn = {
        let mut f = p.declare_function();
        let data = f.declare_local::<i32>();
        f.storage_live(data);
        let ret = f.declare_local::<i32>();
        f.storage_live(ret);
        f.catch_unwind(fn_ptr(try_fn), data, fn_ptr(catch_fn), ret);
        f.assume(eq(load(ret), const_int(0)));
        f.assume(eq(load(data), const_int(42)));
        f.exit();
        p.finish_function(f)
    };

    let p = p.finish_program(main_fn);
    dump_program(p);
    assert_stop::<BasicMem>(p);
}
