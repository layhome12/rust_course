#[path = "course/hello_world.rs"]
mod hello_world;

#[path = "course/variable.rs"]
mod variable;

fn main() {
    // Hello World!
    hello_world::show();
}
