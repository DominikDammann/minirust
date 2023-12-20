use crate::*;

/// tests that if only works with booleans (as it gets translated to a boolean switch)
#[test]
fn if_int_ill_formed() {
    let locals = [];
    let blocks = [
        block!(if_(const_int(0u8), 1, 2)), // ill-formed here at const_int(0u8) as if_ creates boolean cases
        block!(exit()),
        block!(unreachable()),
    ];

    let program = program(&[function(Ret::No, 0, &locals, &blocks)]);
    assert_ill_formed(program);
}

/// tests that the if case can be reached.
/// Also tests that BoolToIntCast converts true to 1.
#[test]
fn if_works() {
    let locals = [];
    let blocks = [
        block!(if_(const_bool(true), 1, 2)),
        block!(exit()),
        block!(unreachable()),
    ];

    let program = program(&[function(Ret::No, 0, &locals, &blocks)]);
    assert_stop(program);
}

/// tests that the else case can be reached.
/// Also tests that BoolToIntCast converts false to 0.
#[test]
fn else_works() {
    let locals = [];
    let blocks = [
        block!(if_(const_bool(false), 1, 2)),
        block!(unreachable()),
        block!(exit()),
    ];

    let program = program(&[function(Ret::No, 0, &locals, &blocks)]);
    assert_stop(program);
}

/// tests that an integer switch that switches on a boolean is ill-formed.
#[test]
fn boolean_switch_is_ill_formed() {
    let locals = [];
    let blocks = [
        block!(switch_int(const_bool(false), &[(0u8, 1)], 1)), // ill-formed here at const_bool as switch_int creates int cases
        block!(unreachable()),
    ];

    let program = program(&[function(Ret::No, 0, &locals, &blocks)]);
    assert_ill_formed(program);
}

/// tests that switch_int can access an arbitrary case and the fallback case.
#[test]
fn switch_int_works() {
    let locals = [];
    let blocks = [
        block!(switch_int(const_int(1u8), &[(0u8, 3), (1u8, 1)], 3)),
        block!(switch_int(const_int(12u8), &[(0u8, 3), (1u8, 3)], 2)),
        block!(exit()),
        block!(unreachable()),
    ];

    let program = program(&[function(Ret::No, 0, &locals, &blocks)]);
    assert_stop(program);
}

/// tests that switching on a enum discriminant is possible
#[test]
fn switch_enum_works() {
    let enum_ty = enum_ty::<u8>(
        &[
            enum_variant(tuple_ty(&[], size(1), align(1)), &[(offset(0), 4)]),
            enum_variant(tuple_ty(&[], size(1), align(1)), &[(offset(0), 2)]),
        ],
        Discriminator::Branch {
            offset: offset(0),
            fallback: GcCow::new(Discriminator::Invalid),
            children: [
                (2, Discriminator::Known(1.into())),
                (4, Discriminator::Known(0.into())),
            ].into_iter().collect()
        },
        size(1),
        align(1)
    );
    let locals = [enum_ty];
    let blocks = [
        block!(
            storage_live(0),
            set_discriminant(local(0), 0),
            switch_int(get_discriminant(local(0)), &[(0u8, 1), (1u8, 2)], 2)
        ),
        block!(exit()),
        block!(unreachable()),
    ];

    let program = program(&[function(Ret::No, 0, &locals, &blocks)]);
    assert_stop(program);
}
