struct Color(i32, i32, i32);

fn main() {
    let my_colour = Color(50, 0, 50); // Make a colour out of RGB (red, green, blue)
    println!("The second part of the colour is: {}", my_colour.1);
}