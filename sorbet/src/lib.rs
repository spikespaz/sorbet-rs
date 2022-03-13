#![warn(missing_docs)]

//! The main crate for creating a [`Window`], drawing [`Primitive`]s to the screen, and handling [`Event`]s.
//! Each member of this top-level module has useful documentation and implementation specifics that you should look into.

pub mod fonts;

#[doc(inline)]
pub use sorbet_color as color;

/// Re-exported from [`lyon::math`], all types here use [`f32`] with the default [`euclid::UnknownUnit`] unit.
pub use lyon::math;

/// This trait marks primitives and widgets that have a known size, or may have
/// their size computed lazily granted that they have a valid reference to a parent
/// and access to the tree that contains them.
///
/// Technically only [`Dimensioned::size()`] needs to be implemented as other provided methods
/// delegate to this to provide their values, but it is recommended to override
/// other methods if an implementor stores the required values differently.
pub trait Dimensioned {
    /// Returns a [`math::Size`] with the `width` and `height` fields for the bounding box
    /// of an implementor.
    fn size(&self) -> math::Size;

    /// Returns the width of the implementor's bounding box.
    ///
    /// By default this returns the `width` field of [`Dimensioned::size()`],
    /// but an implementor may want to change this if they don't store an internal [`math::Size`].
    fn width(&self) -> f32 {
        self.size().width
    }

    /// Returns the height of the implementor's bounding box.
    ///
    /// By default this returns the `height` field of [`Dimensioned::size()`],
    /// but an implementor may want to change this if they don't store an internal [`math::Size`].
    fn height(&self) -> f32 {
        self.size().height
    }
}

/// This trait marks primitives and widgets that have a position in screen-space.
///
/// Technically only [`Positioned::position()`] needs to be implemented as other provided methods
/// delegate to this to provide their values, but it is recommended to override
/// other methods if an implementor stores the required values differently.
pub trait Positioned {
    /// Returns a [`math::Point`] with the `x` and `y` coordinates for the bounding box
    /// of an implementor. This will always be the top-left vertex/coordinate of
    /// the implementor's bounding box.
    fn position(&self) -> math::Point;

    /// Returns the distance from the X-axis of the origin of the screen-space to the left-edge of the bounding box.
    /// The origin of the screen-space is dependant on the surface that the implementor will be rendered to.
    ///
    /// By default this returns the `x` field of [`Positioned::position()`],
    /// but an implementor may want to change this if they don't store an internal [`math::Size`].
    fn x(&self) -> f32 {
        self.position().x
    }

    /// Returns the distance from the Y-axis of the origin of the screen-space to the top-edge of the bounding box.
    /// The origin of the screen-space is dependant on the surface that the implementor will be rendered to.
    ///
    /// By default this returns the `y` field of [`Positioned::position()`],
    /// but an implementor may want to change this if they don't store an internal [`math::Size`].
    fn y(&self) -> f32 {
        self.position().y
    }
}

/// This trait marks types who implement [`Dimensioned`] and [`Positioned`], providing convenience
/// methods for getting various properties of a widget or primitive's bounding box.
/// It is automatically implemented for all types that have the two required traits.
pub trait Bounded: Dimensioned + Positioned {
    /// Returns a [`math::Rect`] with `x`, `y`, `width`, and `height` fields for the bounding box of an implementor.
    /// If a type stores this information in a [`math::Rect`] already, it is recommended to override this and return
    /// that to avoid unnecessary copies.
    fn rect(&self) -> math::Rect {
        math::rect(self.x(), self.y(), self.width(), self.height())
    }

    /// An alias to [`Positioned::x()`].
    fn left(&self) -> f32 {
        self.x()
    }

    /// Returns the distance from the X-axis of the origin of the screen-space to the right-edge of the bounding box.
    /// The origin of the screen-space is dependant on the surface that the implementor will be rendered to.
    ///
    /// See [`Positioned::x()`] and [`Dimensioned::width()`] for implementation details.
    /// The default implementation is the sum of those two.
    fn right(&self) -> f32 {
        self.x() + self.width()
    }

    /// An alias to [`Positioned::y()`].
    fn top(&self) -> f32 {
        self.y()
    }

    /// Returns the distance from the Y-axis of the origin of the screen-space to the bottom-edge of the bounding box.
    /// The origin of the screen-space is dependant on the surface that the implementor will be rendered to.
    ///
    /// See [`Positioned::y()`] and [`Dimensioned::height()`] for implementation details.
    /// The default implementation is the sum of those two.
    fn bottom(&self) -> f32 {
        self.y() + self.height()
    }

    /// An alias to [`Positioned::position()`].
    fn top_left(&self) -> math::Point {
        self.position()
    }

    /// Returns the coordinates as a [`math::Point`] of the top-right vertex of the bounding box.
    fn top_right(&self) -> math::Point {
        math::point(self.right(), self.top())
    }

    /// Returns the coordinates as a [`math::Point`] of the bottom-left vertex of the bounding box.
    fn bottom_left(&self) -> math::Point {
        math::point(self.left(), self.bottom())
    }

    /// Returns the coordinates as a [`math::Point`] of the bottom-right vertex of the bounding box.
    fn bottom_right(&self) -> math::Point {
        math::point(self.right(), self.bottom())
    }
}

impl<T> Bounded for T where T: Dimensioned + Positioned {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
