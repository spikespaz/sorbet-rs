pub mod fonts;

pub mod math {
    pub use lyon::math::*;
}

/// This trait provides convenience methods for getting the edges and corner-points of an implementor's
/// bounding box. See the note on [`Bounded::size()`] and [`Bounded::size_checked()`], some of the methods
/// may be [`unimplemented!()`] and any of the others that depend on these unchecked methods will panic.
/// Any type that implements this trait should clearly state whether or not you should use the unchecked methods.
/// Most implementors should have a known size, even if their size is dynamic, provided that they have a reference
/// the parent and any trees which they are members of.
pub trait Bounded {
    /// The full bounding-box with `x`, `y`, `width`, and `height` components for an implementor.
    ///
    /// **Note:**
    /// > This will panic if the size of the implementor is unknown.
    /// > See the note for [`Bounded::size()`].
    fn rect(&self) -> math::Rect;

    /// The checked and safe version of [`Bounded::rect()`], guaranteed to never panic and return
    /// [`None`] in the event that the size of the implementor is unknown.
    /// See the note for [`Bounded::size_checked()`].
    fn rect_checked(&self) -> Option<math::Rect>;

    /// The left X-axis of the bounding box for an implementor.
    /// This is a convenience method and returns `self.position().x`.
    fn x(&self) -> f32 {
        self.position().x
    }

    /// The top Y-axis of the bounding box for an implementor.
    /// This is a convenience method and returns `self.position().y`.
    fn y(&self) -> f32 {
        self.position().y
    }

    /// The upper-left point of the bounding box for an implementor.
    fn position(&self) -> math::Point;

    /// The width of the bounding box for an implementor.
    /// This is a convenience method and maps to the interior `width` field of `Some(self.size())`.
    ///
    /// See the note for [`Bounded::size_checked()`].
    fn width_checked(&self) -> Option<f32> {
        self.size_checked().map(|size| size.width)
    }

    /// The height of the bounding box for an implementor.
    /// This is a convenience method and maps to the interior `height` field of `Some(self.size())`.
    ///
    /// See the note for [`Bounded::size_checked()`].
    fn height_checked(&self) -> Option<f32> {
        self.size_checked().map(|size| size.height)
    }

    /// The width and height of the bounding box for an implementor.
    /// If the size is unknown, or can only be known after a computation
    /// triggered by a mutable method, this will return [`None`].
    ///
    /// **Note:**
    /// > This should be used rather than [`Bounded::size()`] if the implementor does not know
    /// > its own size until a later condition is met.
    /// > An implementor may mark this method with [`unimplemented!()`] if it is not applicable.
    fn size_checked(&self) -> Option<math::Size>;

    /// The width of the bounding box for an implementor.
    ///
    /// See the note for [`Bounded::size()`].
    fn width(&self) -> f32 {
        self.size().width
    }

    /// The height of the bounding box for an implementor.
    ///
    /// See the note for [`Bounded::size()`].
    fn height(&self) -> f32 {
        self.size().height
    }

    /// The width and height for the bounding box for an implementor.
    ///
    /// **Note:**
    /// > This should be used rather than [`Bounded::size_checked()`] if the implementor
    /// > has a fixed size or if the size can be computed upon initialization.
    /// > An implementor may mark this method with [`unimplemented!()`] if it is not applicable.
    /// >
    /// > These restrictions apply to all methods of a [`Bounded`] type that rely on a known size.
    /// > You may have to calculate any of these values manually with [`Bounded::size_checked()`].
    fn size(&self) -> math::Size;

    /// An alias for [`Bounded::y()`].
    fn left(&self) -> f32 {
        self.x()
    }

    /// An alias for [`Bounded::x()`].
    fn top(&self) -> f32 {
        self.y()
    }

    /// The right Y-axis of the bounding box for an implementor.
    ///
    /// **Note:**
    /// > This will panic if the size of the implementor is unknown.
    /// > See the note for [`Bounded::size()`].
    fn right(&self) -> f32 {
        self.left() + self.width()
    }

    /// The bottom X-axis of the bounding box for an implementor.
    ///
    /// **Note:**
    /// > This will panic if the size of the implementor is unknown.
    /// > See the note for [`Bounded::size()`].
    fn bottom(&self) -> f32 {
        self.top() + self.height()
    }

    /// An alias for [`Bounded::position()`].
    fn top_left(&self) -> math::Point {
        self.position()
    }

    /// The upper-right point of the bounding box for an implementor.
    ///
    /// **Note:**
    /// > This will panic if the size of the implementor is unknown.
    /// > See the note for [`Bounded::size()`].
    fn top_right(&self) -> math::Point {
        math::point(self.right(), self.top())
    }

    /// The bottom-left point of the bounding box for an implementor.
    ///
    /// **Note:**
    /// > This will panic if the size of the implementor is unknown.
    /// > See the note for [`Bounded::size()`].
    fn bottom_left(&self) -> math::Point {
        math::point(self.left(), self.right())
    }

    /// The bottom-right point of the bounding box for an implementor.
    ///
    /// **Note:**
    /// > This will panic if the size of the implementor is unknown.
    /// > See the note for [`Bounded::size()`].
    fn bottom_right(&self) -> math::Point {
        math::point(self.right(), self.bottom())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
