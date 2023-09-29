use crate::rect::Rect;


pub fn rect_overlaps(rect1: &Rect, rect2: &Rect) -> bool {
    rect1.x() < rect2.x() + rect2.width()
        && rect1.x() + rect1.width() > rect2.x()
        && rect1.y() < rect2.y() + rect2.height()
        && rect1.y() + rect1.height() > rect2.y()
}

pub fn rect_contains(rect: &Rect, point: &(f32, f32)) -> bool {
    point.0 >= rect.x()
        && point.0 < rect.x() + rect.width()
        && point.1 >= rect.y()
        && point.1 < rect.y() + rect.height()
}
