fn main() {
    // Using single varialbes to calculate area
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

// Using single variables to calculate area
fn area(width: u32, height: u32) -> u32 {
    width * height
}
