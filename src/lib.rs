#![no_std]
#![allow(unused_imports)]
//!
//! Downcast trait: A module to support downcasting dyn traits using [core::any].
//!
//! Downcast traits enables callers to convert dyn objects that implement the
//! DowncastTrait trait to any trait that is supported by the struct implementing the trait.
//! The most useful usecase for this is if a class contains a list of objects that implements a
//! trait and want to call functions on a subset which implements another trait too. This is similar
//! to casting to a sub-class in an object oriented language.
//!
//! Example:
//! * A Widget trait is implemented for all widgets in a graphical user interface system.
//! * The widget trait extends the DowncastTrait.
//! * A widget may implement the Container trait if it is possible to add child widgets to the widget.
//! * A container has a list of widgets, and want to call a specific functions on all widgets that
//!   implement container.
//!
//! ```
//! #[macro_use] extern crate downcast_trait;
//! use downcast_trait::DowncastTrait;
//! use core::{any::{Any, TypeId}, mem};
//! trait Widget: DowncastTrait {}
//! trait Container: Widget {
//!     fn enumerate_widget_leaves_recursive(&self) -> Vec<&Box<dyn Widget>>;
//! }
//! struct Window {
//!     sub_widgets: Vec<Box<dyn Widget>>,
//! }
//! impl Widget for Window {}
//! impl Container for Window {
//!     fn enumerate_widget_leaves_recursive(&self) -> Vec<&Box<dyn Widget>> {
//!         let mut result = Vec::<&Box<dyn Widget>>::new();
//!         self.sub_widgets.iter().for_each(|sub_widget| {
//!             if let Some(sub_container) =
//!                 downcast_trait!(dyn Container, sub_widget.as_ref().to_downcast_trait())
//!             {
//!                 result.extend(sub_container.enumerate_widget_leaves_recursive());
//!             } else {
//!                 result.push(sub_widget);
//!             }
//!         });
//!         result
//!     }
//! }
//! impl DowncastTrait for Window {
//!     downcast_trait_impl_convert_to!(dyn Container);
//! }
//! ```
use core::{any::{Any, TypeId}, mem};

/// This trait should be implemented by any structs that or traits that should be downcastable
/// to dowcast to one or more traits. The functions required by this trait should be implemented
/// using the [downcast_trait_impl_convert_to<](macro.downcast_trait_impl_convert_to.html) macro.
/// ```ignore
/// trait Widget: DowncastTrait {}
/// ```
pub trait DowncastTrait {
    fn convert_to_trait<'a>(&'a self, trait_id: TypeId) -> Option<&'a (dyn Any)>;
    fn convert_to_trait_mut<'a>(&'a mut self, trait_id: TypeId) -> Option<&'a mut (dyn Any)>;
    fn to_downcast_trait<'a>(&'a self) -> &'a dyn DowncastTrait;
    fn to_downcast_trait_mut<'a>(&'a mut self) -> &'a mut dyn DowncastTrait;
}

/// This macro can be used to cast a &dyn DowncastTrait to an implemented trait e.g:
/// ```ignore
/// if let Some(sub_container) =
///     downcast_trait!(dyn Container, sub_widget.as_ref().to_downcast_trait())
/// {
///   //Use downcasted trait
/// }
/// ```
#[macro_export]
macro_rules! downcast_trait {
    ( dyn $type:path, $src:expr) => {{
        fn transmute_helper<'a>(src: &'a dyn DowncastTrait) -> Option<&'a dyn $type> {
            src.convert_to_trait(TypeId::of::<dyn $type>())
                .map(|dst| unsafe { mem::transmute::<&'a (dyn Any), &'a (dyn $type + 'a)>(dst) })
        }
        transmute_helper($src)
    }};
}

/// This macro can be used to cast a &dyn mut DowncastTrait to an implemented trait e.g:
/// ```ignore
/// if let Some(sub_container) =
///     downcast_trait_mut!(dyn Container, sub_widget.as_ref().to_downcast_trait())
/// {
///   //Use downcasted trait
/// }
/// ```
#[macro_export]
macro_rules! downcast_trait_mut {
    ( dyn $type:path, $src:expr) => {{
        fn transmute_helper<'a>(src: &'a mut dyn DowncastTrait) -> Option<&'a mut dyn $type> {
            src.convert_to_trait_mut(TypeId::of::<dyn $type>())
                .map(|dst| unsafe {
                    mem::transmute::<&'a mut (dyn Any), &'a mut (dyn $type + 'a)>(dst)
                })
        }
        transmute_helper($src)
    }};
}
/// This macro can be used by a struct impl, to implement the functions required by the downcas traitt
/// to dowcast to one or more traits.
/// ```ignore
/// impl DowncastTrait for Window {
///     downcast_trait_impl_convert_to!(dyn Container, dyn Scrollable, dyn Clickable);
/// }
/// ```

#[macro_export]
macro_rules! downcast_trait_impl_convert_to
{
    ($(dyn $type:path),+) => {
        fn convert_to_trait<'a>(&'a self, trait_id: TypeId) -> Option<&'a (dyn Any)> {
            if false
            {
               None
            }
            $(
            else if trait_id == TypeId::of::<dyn $type>()
            {
               unsafe {
                   Some(mem::transmute::<&'a (dyn $type + 'a), &'a dyn Any>(
                       self as &'a (dyn $type + 'a)
                   ))
               }
            }
            )*
            else
            {
                None
            }
        }

        fn convert_to_trait_mut<'a>(&'a mut self, trait_id: TypeId) -> Option<&'a mut (dyn Any)> {
            if false
            {
               None
            }
            $(
            else if trait_id == TypeId::of::<dyn $type>()
            {
               unsafe {
                   Some(mem::transmute::<&'a mut (dyn $type + 'a), &'a mut dyn Any>(
                       self as &'a mut (dyn $type + 'a)
                   ))
               }
            }
            )*
            else
            {
                None
            }
        }

        fn to_downcast_trait<'a>(&'a self) -> &'a dyn DowncastTrait
        {
            self
        }

        fn to_downcast_trait_mut<'a>(&'a mut self) -> &'a mut dyn DowncastTrait
        {
            self
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    trait Downcasted {
        fn get_number(&self) -> u32;
    }
    trait Downcasted2 {
        fn get_number(&self) -> u32;
    }
    struct Downcastable {
        val: u32,
    }
    impl Downcasted for Downcastable {
        fn get_number(&self) -> u32 {
            self.val + 123
        }
    }
    impl Downcasted2 for Downcastable {
        fn get_number(&self) -> u32 {
            self.val + 456
        }
    }
    impl DowncastTrait for Downcastable {
        downcast_trait_impl_convert_to!(dyn Downcasted, dyn Downcasted2);
    }

    #[test]
    fn exploration<'a>() {
        let mut tst = Downcastable { val: 0 };
        let ts: &mut dyn DowncastTrait = tst.to_downcast_trait_mut();
        let downcasted_maybe = downcast_trait!(dyn Downcasted, ts);
        if let Some(downcasted) = downcasted_maybe {
            assert_eq!(downcasted.get_number(), 123);
        }
        //drop(tst);
        let downcasted_maybe2 = downcast_trait!(dyn Downcasted2, ts);
        if let Some(downcasted2) = downcasted_maybe2 {
            assert_eq!(downcasted2.get_number(), 456);
        }

        let mut downcasted_maybemut = downcast_trait_mut!(dyn Downcasted2, ts);
        match &mut downcasted_maybemut {
            Some(downcasted_mut) => {
                assert_eq!(downcasted_mut.get_number(), 456);
            }
            None => assert!(false),
        }
    }
}
