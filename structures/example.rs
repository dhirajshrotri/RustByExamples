struct Point {
    x: f32,
    y: f32
}

struct Reactangle {
    top_left: Point,
    bottom_right: Point
}

fn area_of_rectangle(rect: Reactangle) -> f32 {
    let Reactangle { top_left, bottom_right } = rect;
   

    let len: f32 = bottom_right.x - top_left.x;
    let width: f32 = bottom_right.y - top_left.y;

    len * width
}

fn create_square (point: Point, attr: f32) -> Reactangle {
    let point2: Point = Point { x: point.x + attr, y: point.y + attr };
    Reactangle { top_left: point, bottom_right: point2 }
}

fn main() {
    let point1: Point = Point { x: 10.3, y: 0.4 };
    let point2: Point = Point { x: 15.3, y: 4.4 };
    let new_rect: Reactangle = Reactangle { top_left: point1, bottom_right: point2};

    println!("{}", area_of_rectangle(new_rect));

    //calling create_square
    let point3: Point = Point {x: 5.1, y: 6.2};
    let square: Reactangle = create_square(point3, 5.0);

    println!("{} {}", square.top_left.x, square.top_left.y);
    println!("{} {}", square.bottom_right.x, square.bottom_right.y);
}
