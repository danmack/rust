fn closure_to_loc() {
    let mut x = |c| c + 1;
    x = |c| c + 1;
    //~^ ERROR mismatched types
}

fn closure_from_match() {
    let x = match 1usize {
    //~^ ERROR match arms have incompatible types
        1 => |c| c + 1,
        2 => |c| c - 1,
        _ => |c| c - 1
    };
}

fn main() { }
