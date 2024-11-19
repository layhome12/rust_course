#[path = "course/hello_world.rs"]
mod hello_world;

#[path = "course/variable.rs"]
mod variable;

#[path = "course/tuple_array.rs"]
mod tuple_array;

fn main() {
    // Hello World!
    hello_world::show();
}
