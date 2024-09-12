

macro_rules! cast_component {
    ( $component:expr => $new_type:tt) => {
        $component.as_any().downcast_ref::<$new_type>().unwrap()
    };
    ( $component:expr => $mutable:tt $new_type:tt) => {
        $component.as_any_mut().downcast_mut::<$new_type>().unwrap()
    }
}


pub(crate) use cast_component;
