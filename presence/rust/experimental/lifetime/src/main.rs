// Lifetime annotation explicitly describes the lifetime relationship between references and values.
// It is used by the compiler to borrow check the references (outputs) against their borrowed
// values (inputs).
// If a reference is associated with multiple values by the same annotation, the value of the
// shortest lifetime matters, as shown by the function example below.
//
// For the function below, the annotation 'a associates the output to both inputs of x and y, and
// tells the compiler to borrow check the returned reference against the values referenced by both
// x and y, i.e. the returned reference must not outlive the values referenced by both x and y.
//
// Note1: the same annotation 'a below does not mean x, y, and the output has the same lifetime. It
// only means the output is associated with both x and y.
//
// Note2: both x and y are temporary and only lives within the function, while the values referenced
// by them lives outside the function. The return also lives outside the function.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// For struct, the outputs are the references defined in the struct, the inputs are the values
/// borrowed by all the references.
/// The struct below uses different annotations for x and y to allow x to live longer than y and be
/// accessible by v.
struct Foo<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

fn struct_multiple_lifetimes() {
    let x1 = 1;
    let v;
    let f;
    {
        let y1 = 2;
        f = Foo { x: &x1, y: &y1 };
        v = f.x;
    }
    // f cannot be used here since f.y will outlive y1.
    println!("{}", *v);
}

/// Using the same lifetime annotation tells the compiler the lifetime of x is associated to the
/// values referenced by both x and y. The same applies to the lifetime of y.
/// ```Rust
///  struct Foo<'a> {
///      x: &'a i32,
///      y: &'a i32,
/// }
/// ```
/// i.e. both x and y must mot outlive x1 and y1.
/// Note this is different from function's lifetime annotation since struct fields are part of the
/// output.

// The example below further shows that "borrow check" checks against `value`, which may be passed
// by intermediate references that "borrow check" will ignore.
// Note, the above example "borrow check" both x1 and y1 and use the shorter lifetime for the v.
fn borrow_check_value_only() {
    let value = String::from("hello");
    let reference_2;
    {
        let reference = &value;
        // Borrow check only check against the original value.
        // So reference_2 can outlive reference.
        reference_2 = reference;
    }
    println!("The value of reference_2 is: {}", *reference_2);
}
fn lifetime_annotation() {
    let str1 = String::from("abcd");
    let str2 = String::from("abcde");
    println!("longest string: {}", longest(&str1, &str2));

    let a = A;
    let b = B { a: &a };
    let c = C { a: &a, b: &b };

    struct_multiple_lifetimes()
}

// Shows the four stage of a variable: declare, initialize, use, drop.
// The variable's "active lifetime" is between initialize and use.
// The variable's "max lifetime" is between initialize and drop.
// Borrow check means a reference's "active time" should be shorter than
// the borrowed value's "max lifetime".
fn lifetime_stages() {
    // Declare.
    let x;
    // Initialize.
    x = 1;
    // use.
    println!("{}", x);
    // drop.
}

fn lifetime_borrowed() {
    let ref1;
    let mut value = String::from("value");
    // ref1 lives until it is printed.
    ref1 = &value;
    // Note, ref2 copies from ref1 and borrows the same `value`.
    // ref1 and ref2 "shared borrow" the `value` and their lifetime can be overlapped.
    let ref2 = ref1;
    println!("ref1 borrowed {}", ref1);
    println!("ref2 borrowed {}", ref2);
    // ref3 "exclusively borrow" the `value` and its lifetime cannot overlap with other borrowers.
    let ref3 = &mut value;
    ref3.push('!');
    println!("ref3 borrowed {}", ref3);
    // value can be accessed only when it is returned from all borrowers (out of their lifetime).
    println!("value {}", value);
}

// Lifetime annotations defines the relationship between the lifetimes of a reference and its
// borrowed value, i.e. the borrowed value must outlive the borrowing reference.


struct A;
// The lifetime annotation in struct defines the lifetime relation between the struct instance and
// its reference fields.
// No lifetime annotation elision in struct even though the relationship is obvious.
struct B<'a> {
    a: &'a A,
}
struct C<'a> {
    a: &'a A,
    b: &'a B<'a>,
}

fn show_lifetime_scope() {
    let r;
    {
        let x = 5;
        // r's lifetime is defined by its scope, until its last usage of println.
        // Therefore, the value x outlives r.
        r = &x;
        println!("r: {}", r);
    }
}

fn main() {
    lifetime_stages();
    lifetime_borrowed();
    lifetime_annotation();
    borrow_check_value_only();
    show_lifetime_scope();
}
