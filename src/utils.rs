use gpui::{Pixels, Point, Size, point, px, size};

#[inline]
pub fn dpi_size_to_gpui(logical_size: dpi::LogicalSize<u32>) -> Size<Pixels> {
    size(px(logical_size.width as _), px(logical_size.height as _))
}

#[inline]
pub fn dpi_pos_to_gpui(logical_pos: dpi::LogicalPosition<i32>) -> Point<Pixels> {
    point(px(logical_pos.x as _), px(logical_pos.y as _))
}
