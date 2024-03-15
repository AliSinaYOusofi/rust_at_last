struct User {
    username: String,
    email: String,
    is_online: bool
}

fn build_a_user(username: String, email: String) -> User {
    let new_user = User {
        username,
        email,
        is_online: false
    };

    return new_user
}

struct RGB (i32, i32, i32);
struct RGBA(i32, i32, i32, i32);

fn build_color(i: i32, v: i32, j: i32) -> RGB {
    return RGB(i, v, j);
}

fn build_rgba_color(i: i32, v: i32, j: i32, k: i32) -> RGBA {
    return RGBA(i, v, j, k);
}

struct Rectangle {
    width: u32,
    height: u32
}

fn dimensions(rectangle: Rectangle) -> u32 {
    return rectangle.height * rectangle.width
}