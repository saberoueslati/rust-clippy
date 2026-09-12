#![warn(clippy::stacked_if_match)]
#![feature(postfix_match)]
#![allow(
    clippy::blocks_in_conditions,
    clippy::match_single_binding,
    clippy::needless_bool,
    clippy::redundant_closure_call
)]

fn direct_if(first: bool, second: bool, third: bool) {
    if if first {
        //~^ ERROR: this `if` expression is visually stacked inside another `if` expression
        second
    } else {
        third
    } {
        println!("stacked if");
    }
}

fn binary_ifs(first: bool, second: bool, third: bool, fourth: bool) {
    if if first { second } else { third } && fourth {
        //~^ ERROR: this `if` expression is visually stacked inside another `if` expression
        println!("left-hand side");
    }

    if fourth == if first { second } else { third } && second {
        //~^ ERROR: this `if` expression is visually stacked inside another `if` expression
        println!("right-hand side");
    }

    if if first { 1 } else { 2 } == 1 {
        //~^ ERROR: this `if` expression is visually stacked inside another `if` expression
        println!("non-boolean inner if");
    }

    if first && second && third && fourth && if first { second } else { third } {
        //~^ ERROR: this `if` expression is visually stacked inside another `if` expression
        println!("long binary condition");
    }
}

#[rustfmt::skip]
fn hoisting_would_change_behavior(values: &[i32], option: Option<i32>) {
    // Hoisting the inner `if` would evaluate `values[0]` before the emptiness check.
    if !values.is_empty() && if values[0] == 0 { true } else { false } {
        //~^ ERROR: this `if` expression is visually stacked inside another `if` expression
        println!("short-circuit guard");
    }

    // Hoisting the inner `if` would move `number` out of scope.
    if let Some(number) = option && if number > 0 { true } else { false } {
        //~^ ERROR: this `if` expression is visually stacked inside another `if` expression
        println!("let chain binding");
    }
}

fn if_let(option: Option<i32>) {
    if if let Some(number) = option {
        //~^ ERROR: this `if` expression is visually stacked inside another `if` expression
        number > 0
    } else {
        false
    } {
        println!("if let");
    }
}

fn direct_match(value: u8) {
    match match value {
        //~^ ERROR: this `match` expression is visually stacked inside another `match` expression
        0 => 1,
        _ => 2,
    } {
        1 => println!("one"),
        _ => println!("other"),
    }
}

fn borrowing_match(value: Option<String>) {
    // The inner `match` borrows from a scrutinee temporary, so hoisting only the inner `match` into a
    // `let` would drop the temporary too early; the scrutinee must be bound to a local first.
    match match Some(String::from("hello")) {
        //~^ ERROR: this `match` expression is visually stacked inside another `match` expression
        Some(ref text) => text,
        None => return,
    } {
        text => println!("{text}"),
    }

    // Borrowing from a place rather than a temporary; hoisting is fine here, but the help is the same.
    match match value {
        //~^ ERROR: this `match` expression is visually stacked inside another `match` expression
        Some(ref text) => text,
        None => return,
    } {
        text => println!("{text}"),
    }
}

fn binary_matches(value: u8) {
    match match value {
        //~^ ERROR: this `match` expression is visually stacked inside another `match` expression
        0 => 1,
        _ => 2,
    } + 1
    {
        1 => println!("one"),
        _ => println!("other"),
    }

    match 1 + match value {
        //~^ ERROR: this `match` expression is visually stacked inside another `match` expression
        0 => 1,
        _ => 2,
    } {
        1 => println!("one"),
        _ => println!("other"),
    }
}

const STACKED_IN_CONST: bool = if if true { false } else { true } {
    //~^ ERROR: this `if` expression is visually stacked inside another `if` expression
    true
} else {
    false
};

fn triple_if(first: bool, second: bool, third: bool) {
    if if if first {
        //~^ ERROR: this `if` expression is visually stacked inside another `if` expression
        //~^^ ERROR: this `if` expression is visually stacked inside another `if` expression
        second
    } else {
        third
    } {
        second
    } else {
        third
    } {
        println!("triple if");
    }
}

fn no_lint(first: bool, second: bool, third: bool, value: u8) {
    if first {
        println!("ordinary if");
    }

    if (if first { second } else { third }) {
        println!("parenthesized if");
    }

    if match value {
        0 => first,
        _ => second,
    } {
        println!("if match");
    }

    match if first { 1 } else { 2 } {
        1 => println!("match if"),
        _ => println!("other"),
    }

    if bool_identity(if first { second } else { third }) {
        println!("if in call");
    }

    if { if first { second } else { third } } {
        println!("if in block");
    }

    if (|| if first { second } else { third })() {
        println!("if in closure");
    }

    match (match value {
        0 => 1,
        _ => 2,
    }) {
        1 => println!("parenthesized match"),
        _ => println!("other"),
    }
}

fn multiline_conditions_no_lint(
    first_condition: bool,
    second_condition: bool,
    third_condition: bool,
    fourth_condition: bool,
    flag: bool,
    value: Option<i32>,
) {
    if first_condition
        && second_condition
        && third_condition
        && fourth_condition
        && if flag {
            first_condition && second_condition
        } else {
            true
        }
    {
        println!("multiline condition");
    }

    if let Some(number) = value
        && first_condition
        && second_condition
        && third_condition
        && fourth_condition
        && number > 0
        && if flag { number < 100 } else { true }
    {
        println!("multiline let chain");
    }
}

fn postfix_matches_no_lint(value: u8) {
    match value.match {
        0 => 1,
        _ => 2,
    } {
        1 => println!("postfix inner match"),
        _ => println!("other"),
    }

    value.match {
        0 => 1,
        _ => 2,
    }.match {
        1 => println!("postfix outer match"),
        _ => println!("other"),
    };
}

#[rustfmt::skip]
fn separated_if_no_lint(first: bool, second: bool, third: bool) {
    if if first {
        second
    } else {
        third
    }

    {
        println!("separated if");
    }
}

fn bool_identity(value: bool) -> bool {
    value
}

macro_rules! outer_if {
    () => {
        if if true { false } else { true } {
            println!("macro outer");
        }
    };
}

macro_rules! inner_if {
    () => {
        if true { false } else { true }
    };
}

fn macros() {
    outer_if!();

    if inner_if!() {
        println!("macro condition");
    }
}

fn main() {
    direct_if(true, false, true);
    binary_ifs(true, false, true, false);
    hoisting_would_change_behavior(&[1], Some(1));
    if_let(Some(1));
    direct_match(1);
    borrowing_match(Some(String::from("world")));
    binary_matches(1);
    triple_if(true, false, true);
    no_lint(true, false, true, 1);
    multiline_conditions_no_lint(true, false, true, false, true, Some(1));
    postfix_matches_no_lint(1);
    separated_if_no_lint(true, false, true);
    macros();
    let _ = STACKED_IN_CONST;
}
