fn main() {
    // Using single varialbes to calculate area
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // Using tuple to group width and height
    let rect1 = (30, 50); 
    
    println!(
        "The area of the rectangle using TUPLES is {} square pixels.",
        area_using_tuples(rect1)
    );
}

// Using single variables to calculate area
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// Using tuple to calculate area
fn area_using_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
