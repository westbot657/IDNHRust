

macro_rules! cast_component {
    ( $component:expr => $new_type:tt) => {
        $component.as_any().downcast_ref::<$new_type>().unwrap()
    };
    ( $component:expr => $mutable:tt $new_type:tt) => {
        $component.as_any_mut().downcast_mut::<$new_type>().unwrap()
    }
}


macro_rules! SETTINGS {
    ( text color 4 u8 ) => {
        (200, 200, 200, 255)
    };
    ( text color 4 f32 ) => {
        (0.784313725490196, 0.784313725490196, 0.784313725490196, 1.0)
    };


    ( bg dark 4 u8 ) => {
        (24, 24, 24, 255)
    };
    ( bg dark 4 f32 ) => {
        (0.0941176470588235, 0.0941176470588235, 0.0941176470588235, 1.0)
    };


    ( bg medium 4 u8 ) => {
        (31, 31, 31, 255)
    };
    ( bg medium 4 f32 ) => {
        (0.12156862745098, 0.12156862745098, 0.12156862745098, 1.0)
    };


    ( bg light 4 u8 ) => {
        (100, 100, 100, 255)
    };


    ( text highlight 4 u8 ) => {
        (0, 122, 204, 127)
    };


    ( text link 4 u8 ) => {
        (0, 122, 204, 255)
    }
}


macro_rules! CONST {
    ( text height ) => {
        50
    };
    ( atlas ) => {
        4096
    }
}


pub(crate) use {cast_component, SETTINGS, CONST};
