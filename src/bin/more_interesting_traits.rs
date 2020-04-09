// Taken from chapter 17 of the Rust Programming Language, by Steve Klabnik and Carol Nichols. You
// guys rule!

trait Draw {
    fn draw(&self);
}

struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

struct Button {
    size: i32,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drew a button of size {}", self.size);
    }
}

struct Widget {
    circumference: f32,
    rotation: f32,
}

impl Draw for Widget {
    fn draw(&self) {
        println!(
            "Drew a widget of circumference {}, rotated by {} degrees",
            self.circumference, self.rotation
        );
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button { size: 14 }),
            Box::new(Widget {
                circumference: 14.2,
                rotation: 79.3,
            }),
            Box::new(Button { size: 27 }),
        ],
    };

    screen.run();
}
