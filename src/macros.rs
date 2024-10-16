

macro_rules! cast_component {
    ( $component:expr => $new_type:tt) => {
        $component.as_any().downcast_ref::<$new_type>().unwrap()
    };
    ( $component:expr => mut $new_type:tt) => {
        $component.as_any_mut().downcast_mut::<$new_type>().unwrap()
    }
}


macro_rules! SETTINGS {
    ( text color 4 u8 ) => {
        (200, 200, 200, 255)
    };
    ( text color 4 f32 ) => {
        (0.78431373, 0.78431373, 0.78431373, 1.0)
    };


    ( bg dark 4 u8 ) => {
        (24, 24, 24, 255)
    };
    ( bg dark 4 f32 ) => {
        (0.09411765, 0.09411765, 0.09411765, 1.0)
    };


    ( bg medium 4 u8 ) => {
        (31, 31, 31, 255)
    };
    ( bg medium 4 f32 ) => {
        (0.12156863, 0.12156863, 0.12156863, 1.0)
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
        32
    };
    ( text height f32 ) => {
        32.0
    };
    ( atlas ) => {
        4096
    };
    ( atlas f32 ) => {
        4096.0
    };
    ( text atlas ) => {
        2048
    };
    ( text atlas f32 ) => {
        2048.0
    };
}

macro_rules! collides {
    ( $app:expr, $component:expr, $point:expr) => {
        $app.collides(($component.position.0, $component.position.1, $component.size.0, $component.size.1), $point)
    };
}

macro_rules! font_size {
    ( $sz:expr ) => {
        $sz / 32.0
    };
}

pub(crate) use {cast_component, SETTINGS, CONST, collides, font_size};
