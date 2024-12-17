use std::ops::Add;

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
// The same annotation 'a below does not mean x, y, and the output has the same lifetime.
// It only means the output reference's lifetime is related to both x and y
// (by annotated the same 'a), i.e. the output reference must not outlive the values referenced by
// both x and y.
// Note, both x and y are temporary and only lives within the function, while the values referenced
// by them lives outside the function. The return also lives outside the function.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct A;
// The lifetime annotation in struct defines the lifetime relation between the struct instance and
// its reference fields.
// No lifetime annotation elision in struct even though the relationship is obvious.
struct B<'a> {
    a: &'a A,
}

// Any difference we only use 'a.
// Instance of struct C outlives the value referenced by b.
// b outlives te value referenced by a.
// This also means instance outlives of values referenced by both a and be.
// the same result as using only 'a below.
//struct C<'a, 'b> {
//    a: &'a A,
//    b: &'b B<'a>,
//}

struct C<'a> {
    a: &'a A,
    b: &'a B<'a>,
}

// Using the same lifetime annotation tells the compiler the lifetime of struct instance, x, and y
// are all related, i.e. all must mot outlive the same shorter values that x and y reference to.
// Note this is different from function's lifetime annotation since struct fields are part of the
// output.
// This will break the println codes below which requires x to outlive y.
// Using different lifetime annotation for each field means the instance of the struct must not
// outlive the values which are referenced by each field. Meanwhile, the lifetimes between fields
// are unrelated.
struct Foo<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

fn struct_multiple_lifetimes() {
    let x = 1;
    let v;
    {
        let y = 2;
        let f = Foo { x: &x, y: &y };
        v = f.x;
    }
    println!("{}", *v);
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



fn borrow_check_value_only() {
    let value = String::from("hello");
    let reference_2;
    {
        let reference = &value;
        // Borrow check only check against the original value.
        // So reference_2 can outlive reference.
        reference_2 = reference;
    }
    println!("The value of reference_2 is: {}", reference_2);
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
