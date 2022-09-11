#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{ CString, CStr };

#[doc = " Event types"]
pub const projectMEvent_PROJECTM_KEYUP: projectMEvent = 0;
#[doc = " Event types"]
pub const projectMEvent_PROJECTM_KEYDOWN: projectMEvent = 1;
#[doc = " Event types"]
pub const projectMEvent_PROJECTM_VIDEORESIZE: projectMEvent = 2;
#[doc = " Event types"]
pub const projectMEvent_PROJECTM_VIDEOQUIT: projectMEvent = 3;
pub type projectMEvent = u32;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_RETURN: projectMKeycode = 0;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_RIGHT: projectMKeycode = 1;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_LEFT: projectMKeycode = 2;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_UP: projectMKeycode = 3;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_DOWN: projectMKeycode = 4;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_PAGEUP: projectMKeycode = 5;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_PAGEDOWN: projectMKeycode = 6;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_INSERT: projectMKeycode = 7;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_DELETE: projectMKeycode = 8;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_ESCAPE: projectMKeycode = 9;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_LSHIFT: projectMKeycode = 10;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_RSHIFT: projectMKeycode = 11;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_CAPSLOCK: projectMKeycode = 12;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_LCTRL: projectMKeycode = 13;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_HOME: projectMKeycode = 14;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_END: projectMKeycode = 15;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_BACKSPACE: projectMKeycode = 16;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_SLASH: projectMKeycode = 17;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_BACKSLASH: projectMKeycode = 18;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_F1: projectMKeycode = 19;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_F2: projectMKeycode = 20;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_F3: projectMKeycode = 21;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_F4: projectMKeycode = 22;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_F5: projectMKeycode = 23;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_F6: projectMKeycode = 24;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_F7: projectMKeycode = 25;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_F8: projectMKeycode = 26;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_F9: projectMKeycode = 27;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_F10: projectMKeycode = 28;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_F11: projectMKeycode = 29;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_F12: projectMKeycode = 30;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_0: projectMKeycode = 48;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_1: projectMKeycode = 49;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_2: projectMKeycode = 50;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_3: projectMKeycode = 51;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_4: projectMKeycode = 52;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_5: projectMKeycode = 53;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_6: projectMKeycode = 54;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_7: projectMKeycode = 55;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_8: projectMKeycode = 56;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_9: projectMKeycode = 57;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_A: projectMKeycode = 65;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_B: projectMKeycode = 66;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_C: projectMKeycode = 67;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_D: projectMKeycode = 68;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_E: projectMKeycode = 69;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_F: projectMKeycode = 70;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_G: projectMKeycode = 71;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_H: projectMKeycode = 72;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_I: projectMKeycode = 73;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_J: projectMKeycode = 74;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_K: projectMKeycode = 75;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_L: projectMKeycode = 76;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_M: projectMKeycode = 77;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_N: projectMKeycode = 78;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_O: projectMKeycode = 79;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_P: projectMKeycode = 80;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_Q: projectMKeycode = 81;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_R: projectMKeycode = 82;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_S: projectMKeycode = 83;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_T: projectMKeycode = 84;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_U: projectMKeycode = 85;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_V: projectMKeycode = 86;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_W: projectMKeycode = 87;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_X: projectMKeycode = 88;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_Y: projectMKeycode = 89;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_Z: projectMKeycode = 90;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_a: projectMKeycode = 97;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_b: projectMKeycode = 98;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_c: projectMKeycode = 99;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_d: projectMKeycode = 100;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_e: projectMKeycode = 101;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_f: projectMKeycode = 102;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_g: projectMKeycode = 103;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_h: projectMKeycode = 104;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_i: projectMKeycode = 105;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_j: projectMKeycode = 106;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_k: projectMKeycode = 107;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_l: projectMKeycode = 108;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_m: projectMKeycode = 109;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_n: projectMKeycode = 110;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_o: projectMKeycode = 111;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_p: projectMKeycode = 112;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_q: projectMKeycode = 113;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_r: projectMKeycode = 114;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_s: projectMKeycode = 115;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_t: projectMKeycode = 116;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_u: projectMKeycode = 117;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_v: projectMKeycode = 118;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_w: projectMKeycode = 119;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_x: projectMKeycode = 120;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_y: projectMKeycode = 121;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_z: projectMKeycode = 122;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_NONE: projectMKeycode = 123;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_PLUS: projectMKeycode = 124;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_MINUS: projectMKeycode = 125;
#[doc = " Keycodes"]
pub const projectMKeycode_PROJECTM_K_EQUALS: projectMKeycode = 126;
pub type projectMKeycode = u32;
#[doc = " Modifiers"]
pub const projectMModifier_PROJECTM_KMOD_NONE: projectMModifier = -1;
#[doc = " Modifiers"]
pub const projectMModifier_PROJECTM_KMOD_LSHIFT: projectMModifier = 0;
#[doc = " Modifiers"]
pub const projectMModifier_PROJECTM_KMOD_RSHIFT: projectMModifier = 1;
#[doc = " Modifiers"]
pub const projectMModifier_PROJECTM_KMOD_CAPS: projectMModifier = 2;
#[doc = " Modifiers"]
pub const projectMModifier_PROJECTM_KMOD_LCTRL: projectMModifier = 3;
#[doc = " Modifiers"]
pub const projectMModifier_PROJECTM_KMOD_RCTRL: projectMModifier = 4;
pub type projectMModifier = i32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct projectm {
    _unused: [u8; 0],
}
pub type projectm_handle = *mut projectm;
#[doc = " @brief projectM instance settings."]
#[doc = ""]
#[doc = " <p>Use this struct to provide settings for projectM, for example if your application handles projectM configuration"]
#[doc = " internally instead of using the default configuration file.</p>"]
#[doc = ""]
#[doc = " <p>Always allocate the struct using the projectm_alloc_settings() and free it with the projectm_free_settings()"]
#[doc = " function.</p>"]
#[doc = ""]
#[doc = " <p>To allocate memory for char* members, always use projectm_alloc_string(). If any pointer is not NULL,"]
#[doc = " projectm_free_settings() will automatically call projectm_free_string() on it. If you free it on your own, remember"]
#[doc = " to reset the pointer to NULL after doing so!</p>"]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct projectm_settings_s {
    #[doc = "!< Per-pixel mesh X resolution."]
    pub mesh_x: i32,
    #[doc = "!< Per-pixel mesh Y resolution."]
    pub mesh_y: i32,
    #[doc = "!< Target rendering frames per second."]
    pub fps: i32,
    #[doc = "!< Size of the render texture. Must be a power of 2."]
    pub texture_size: i32,
    #[doc = "!< Width of the rendering viewport."]
    pub window_width: i32,
    #[doc = "!< Height of the rendering viewport."]
    pub window_height: i32,
    #[doc = "!< Path to a preset playlist in XML format to be loaded. Use FLAG_DISABLE_PLAYLIST_LOAD to skip loading a playlist."]
    pub preset_url: *mut ::std::os::raw::c_char,
    #[doc = "!< Path to the \"title\" font that is used to render the preset name."]
    pub title_font_url: *mut ::std::os::raw::c_char,
    #[doc = "!< Path to the \"menu\" font that is used to render the built-in on-screen menu."]
    pub menu_font_url: *mut ::std::os::raw::c_char,
    #[doc = "!< Path to data files like default fonts and presets."]
    pub data_dir: *mut ::std::os::raw::c_char,
    #[doc = "!< Display duration for each preset in seconds."]
    pub preset_duration: f64,
    #[doc = "!< Blend-over duration between two presets in seconds."]
    pub soft_cut_duration: f64,
    #[doc = "!< Minimum time in seconds a preset is displayed before a hard cut can happen."]
    pub hard_cut_duration: f64,
    #[doc = "!< Set to true to enable fast beat-driven preset switches."]
    pub hard_cut_enabled: bool,
    #[doc = "!< Beat sensitivity value that must be surpassed for a hard cut."]
    pub hard_cut_sensitivity: f32,
    #[doc = "!< Beat sensitivity. Standard sensitivity is 1.0."]
    pub beat_sensitivity: f32,
    #[doc = "!< Use aspect ration correction in presets that support it."]
    pub aspect_correction: bool,
    #[doc = "!< Used as the \"sigma\" value for a gaussian RNG to randomize preset duration. Unused on Windows."]
    pub easter_egg: f32,
    #[doc = "!< Enable playlist shuffle, selecting a random preset on each switch instead of the next in list."]
    pub shuffle_enabled: bool,
    #[doc = "!< If true, use soft cut ratings on soft cuts and hard cut ratings on hard cuts. If false, the hard cut rating is always used."]
    pub soft_cut_ratings_enabled: bool,
}
#[test]
fn bindgen_test_layout_projectm_settings_s() {
    assert_eq!(
        ::std::mem::size_of::<projectm_settings_s>(),
        104usize,
        concat!("Size of: ", stringify!(projectm_settings_s))
    );
    assert_eq!(
        ::std::mem::align_of::<projectm_settings_s>(),
        8usize,
        concat!("Alignment of ", stringify!(projectm_settings_s))
    );
    fn test_field_mesh_x() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<projectm_settings_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mesh_x) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(projectm_settings_s),
                "::",
                stringify!(mesh_x)
            )
        );
    }
    test_field_mesh_x();
    fn test_field_mesh_y() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<projectm_settings_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mesh_y) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(projectm_settings_s),
                "::",
                stringify!(mesh_y)
            )
        );
    }
    test_field_mesh_y();
    fn test_field_fps() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<projectm_settings_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).fps) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(projectm_settings_s),
                "::",
                stringify!(fps)
            )
        );
    }
    test_field_fps();
    fn test_field_texture_size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<projectm_settings_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).texture_size) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(projectm_settings_s),
                "::",
                stringify!(texture_size)
            )
        );
    }
    test_field_texture_size();
    fn test_field_window_width() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<projectm_settings_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).window_width) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(projectm_settings_s),
                "::",
                stringify!(window_width)
            )
        );
    }
    test_field_window_width();
    fn test_field_window_height() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<projectm_settings_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).window_height) as usize - ptr as usize
            },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(projectm_settings_s),
                "::",
                stringify!(window_height)
            )
        );
    }
    test_field_window_height();
    fn test_field_preset_url() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<projectm_settings_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).preset_url) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(projectm_settings_s),
                "::",
                stringify!(preset_url)
            )
        );
    }
    test_field_preset_url();
    fn test_field_title_font_url() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<projectm_settings_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).title_font_url) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(projectm_settings_s),
                "::",
                stringify!(title_font_url)
            )
        );
    }
    test_field_title_font_url();
    fn test_field_menu_font_url() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<projectm_settings_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).menu_font_url) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(projectm_settings_s),
                "::",
                stringify!(menu_font_url)
            )
        );
    }
    test_field_menu_font_url();
    fn test_field_data_dir() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<projectm_settings_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).data_dir) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(projectm_settings_s),
                "::",
                stringify!(data_dir)
            )
        );
    }
    test_field_data_dir();
    fn test_field_preset_duration() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<projectm_settings_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).preset_duration) as usize - ptr as usize
            },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(projectm_settings_s),
                "::",
                stringify!(preset_duration)
            )
        );
    }
    test_field_preset_duration();
    fn test_field_soft_cut_duration() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<projectm_settings_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).soft_cut_duration) as usize - ptr as usize
            },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(projectm_settings_s),
                "::",
                stringify!(soft_cut_duration)
            )
        );
    }
    test_field_soft_cut_duration();
    fn test_field_hard_cut_duration() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<projectm_settings_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).hard_cut_duration) as usize - ptr as usize
            },
            72usize,
            concat!(
                "Offset of field: ",
                stringify!(projectm_settings_s),
                "::",
                stringify!(hard_cut_duration)
            )
        );
    }
    test_field_hard_cut_duration();
    fn test_field_hard_cut_enabled() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<projectm_settings_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).hard_cut_enabled) as usize - ptr as usize
            },
            80usize,
            concat!(
                "Offset of field: ",
                stringify!(projectm_settings_s),
                "::",
                stringify!(hard_cut_enabled)
            )
        );
    }
    test_field_hard_cut_enabled();
    fn test_field_hard_cut_sensitivity() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<projectm_settings_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).hard_cut_sensitivity) as usize - ptr as usize
            },
            84usize,
            concat!(
                "Offset of field: ",
                stringify!(projectm_settings_s),
                "::",
                stringify!(hard_cut_sensitivity)
            )
        );
    }
    test_field_hard_cut_sensitivity();
    fn test_field_beat_sensitivity() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<projectm_settings_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).beat_sensitivity) as usize - ptr as usize
            },
            88usize,
            concat!(
                "Offset of field: ",
                stringify!(projectm_settings_s),
                "::",
                stringify!(beat_sensitivity)
            )
        );
    }
    test_field_beat_sensitivity();
    fn test_field_aspect_correction() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<projectm_settings_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).aspect_correction) as usize - ptr as usize
            },
            92usize,
            concat!(
                "Offset of field: ",
                stringify!(projectm_settings_s),
                "::",
                stringify!(aspect_correction)
            )
        );
    }
    test_field_aspect_correction();
    fn test_field_easter_egg() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<projectm_settings_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).easter_egg) as usize - ptr as usize
            },
            96usize,
            concat!(
                "Offset of field: ",
                stringify!(projectm_settings_s),
                "::",
                stringify!(easter_egg)
            )
        );
    }
    test_field_easter_egg();
    fn test_field_shuffle_enabled() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<projectm_settings_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).shuffle_enabled) as usize - ptr as usize
            },
            100usize,
            concat!(
                "Offset of field: ",
                stringify!(projectm_settings_s),
                "::",
                stringify!(shuffle_enabled)
            )
        );
    }
    test_field_shuffle_enabled();
    fn test_field_soft_cut_ratings_enabled() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<projectm_settings_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).soft_cut_ratings_enabled) as usize - ptr as usize
            },
            101usize,
            concat!(
                "Offset of field: ",
                stringify!(projectm_settings_s),
                "::",
                stringify!(soft_cut_ratings_enabled)
            )
        );
    }
    test_field_soft_cut_ratings_enabled();
}
#[doc = " @brief projectM instance settings."]
#[doc = ""]
#[doc = " <p>Use this struct to provide settings for projectM, for example if your application handles projectM configuration"]
#[doc = " internally instead of using the default configuration file.</p>"]
#[doc = ""]
#[doc = " <p>Always allocate the struct using the projectm_alloc_settings() and free it with the projectm_free_settings()"]
#[doc = " function.</p>"]
#[doc = ""]
#[doc = " <p>To allocate memory for char* members, always use projectm_alloc_string(). If any pointer is not NULL,"]
#[doc = " projectm_free_settings() will automatically call projectm_free_string() on it. If you free it on your own, remember"]
#[doc = " to reset the pointer to NULL after doing so!</p>"]
pub type projectm_settings = projectm_settings_s;
#[doc = "!< No flags."]
pub const projectm_flags_PROJECTM_FLAG_NONE: projectm_flags = 0;
#[doc = "!< Set this flag to disable loading a preset playlist on startup."]
pub const projectm_flags_PROJECTM_FLAG_DISABLE_PLAYLIST_LOAD: projectm_flags = 1;
#[doc = " Flags that influence projectM instance creation."]
pub type projectm_flags = u32;
pub const projectm_channels_PROJECTM_MONO: projectm_channels = 1;
pub const projectm_channels_PROJECTM_STEREO: projectm_channels = 2;
#[doc = " For specifying audio data format."]
pub type projectm_channels = u32;
#[doc = "!< Rating for hard cuts."]
pub const projectm_preset_rating_type_PROJECTM_HARD_CUT_RATING_TYPE: projectm_preset_rating_type =
    0;
#[doc = "!< Rating for soft cuts."]
pub const projectm_preset_rating_type_PROJECTM_SOFT_CUT_RATING_TYPE: projectm_preset_rating_type =
    1;
#[doc = " Rating types supported by projectM. Used to control preset selection for different types"]
#[doc = " of transitions (hard/soft)."]
pub type projectm_preset_rating_type = u32;
#[doc = "!< Left audio channel."]
pub const projectm_pcm_channel_PROJECTM_CHANNEL_L: projectm_pcm_channel = 0;
#[doc = "!< Left audio channel."]
pub const projectm_pcm_channel_PROJECTM_CHANNEL_0: projectm_pcm_channel = 0;
#[doc = "!< Right audio channel."]
pub const projectm_pcm_channel_PROJECTM_CHANNEL_R: projectm_pcm_channel = 1;
#[doc = "!< Right audio channel."]
pub const projectm_pcm_channel_PROJECTM_CHANNEL_1: projectm_pcm_channel = 1;
#[doc = " Placeholder values that can be used to address channel indices in PCM data arrays."]
pub type projectm_pcm_channel = u32;
#[doc = "!< Random waveform type."]
pub const projectm_touch_type_PROJECTM_TOUCH_TYPE_RANDOM: projectm_touch_type = 0;
#[doc = "!< Draws a circular waveform."]
pub const projectm_touch_type_PROJECTM_TOUCH_TYPE_CIRCLE: projectm_touch_type = 1;
#[doc = "!< Draws a radial blob waveform."]
pub const projectm_touch_type_PROJECTM_TOUCH_TYPE_RADIAL_BLOB: projectm_touch_type = 2;
#[doc = "!< Draws a blob-style waveform."]
pub const projectm_touch_type_PROJECTM_TOUCH_TYPE_BLOB2: projectm_touch_type = 3;
#[doc = "!< Draws another blob-style waveform."]
pub const projectm_touch_type_PROJECTM_TOUCH_TYPE_BLOB3: projectm_touch_type = 4;
#[doc = "!< Draws a derivative-line waveform."]
pub const projectm_touch_type_PROJECTM_TOUCH_TYPE_DERIVATIVE_LINE: projectm_touch_type = 5;
#[doc = "!< Draws a five-blob waveform."]
pub const projectm_touch_type_PROJECTM_TOUCH_TYPE_BLOB5: projectm_touch_type = 6;
#[doc = "!< Draws a single-line waveform."]
pub const projectm_touch_type_PROJECTM_TOUCH_TYPE_LINE: projectm_touch_type = 7;
#[doc = "!< Draws a double-line waveform."]
pub const projectm_touch_type_PROJECTM_TOUCH_TYPE_DOUBLE_LINE: projectm_touch_type = 8;
#[doc = " Waveform render types used in the touch start method."]
pub type projectm_touch_type = u32;

pub fn projectm_alloc_string(string: String) -> *const i8 {
    extern "C" {
        #[doc = " @brief Allocates memory for a string and returns the pointer."]
        #[doc = ""]
        #[doc = " To free the allocated memory, call projectm_free_string(). Do not use free()!"]
        #[doc = ""]
        #[doc = " @return A pointer to a zero-initialized memory area."]
        pub fn projectm_alloc_string(length: u32) -> *mut ::std::os::raw::c_char;
    }

    let len = string.len();

    return unsafe { projectm_alloc_string(len.try_into().unwrap()) };
}

pub fn projectm_free_string(string_: *const i8) {
    extern "C" {
        #[doc = " @brief Frees the memory of an allocated string."]
        #[doc = ""]
        #[doc = " <p>Frees the memory allocated by a call to projectm_alloc_string() or any"]
        #[doc = " (const) char* pointers returned by a projectM API call.</p>"]
        #[doc = ""]
        #[doc = " <p>Do not use free() to delete the pointer!</p>"]
        #[doc = ""]
        #[doc = " @param settings A pointer returned by projectm_alloc_string()."]
        pub fn projectm_free_string(string_: *const ::std::os::raw::c_char);
    }

    unsafe {
        projectm_free_string(string_)
    }
}

pub fn projectm_alloc_settings() -> *mut projectm_settings {
    extern "C" {
        #[doc = " @brief Allocates memory for a projectm_settings struct and returns the pointer."]
        #[doc = ""]
        #[doc = " <p>This will not allocate memory for the char* members in the struct. These will be set to NULL initially."]
        #[doc = " To store a string in these members, use projectm_alloc_string() to allocate the required memory. Do not use"]
        #[doc = " malloc()!</p>"]
        #[doc = ""]
        #[doc = " <p>To free the allocated memory, call projectm_free_settings(). Do not use free()!</p>"]
        #[doc = ""]
        #[doc = " @return A pointer to a zero-initialized projectm_settings struct."]
        pub fn projectm_alloc_settings() -> *mut projectm_settings;
    }

    return unsafe {
        projectm_alloc_settings()
    }
}

pub fn projectm_free_settings(settings_: *const projectm_settings) {
    extern "C" {
        #[doc = " @brief Frees the memory of an allocated projectm_settings structure."]
        #[doc = ""]
        #[doc = " <p>Frees the memory allocated by a call to projectm_alloc_settings() or any"]
        #[doc = " projectm_settings pointer returned by an API call. Any non-null char pointers"]
        #[doc = " will also be free'd using projectm_free_string().</p>"]
        #[doc = ""]
        #[doc = " <p>Do not use free() to delete the pointer!</p>"]
        #[doc = ""]
        #[doc = " @param settings A pointer returned by projectm_alloc_settings()."]
        pub fn projectm_free_settings(settings_: *const projectm_settings);
    }

    unsafe {
        projectm_free_settings(settings_)
    }
}

#[doc = " @brief Callback function that is executed on each preset change."]
#[doc = ""]
#[doc = " Can be used for example to update the application window title."]
#[doc = ""]
#[doc = " @param is_hard_cut True if the preset was switched using a hard cut via beat detection."]
#[doc = " @param index The playlist index of the new preset."]
#[doc = " @param user_data A user-defined data pointer that was provided when registering the callback,"]
#[doc = "                  e.g. context information."]
pub type projectm_preset_switched_event = ::std::option::Option<
    unsafe extern "C" fn(
        is_hard_cut: bool,
        index: u32,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;
#[doc = " @brief Callback function that is executed is the shuffle setting has changed."]
#[doc = " @param shuffle_enabled True if shuffle is enabled, false if it was disabled."]
#[doc = " @param user_data A user-defined data pointer that was provided when registering the callback,"]
#[doc = "                  e.g. context information."]
pub type projectm_shuffle_enable_changed_event = ::std::option::Option<
    unsafe extern "C" fn(shuffle_enabled: bool, user_data: *mut ::std::os::raw::c_void),
>;
#[doc = " @brief Callback function that is executed if a preset change failed."]
#[doc = ""]
#[doc = " The message pointer is only valid inside the callback. Make a copy if it must be kept"]
#[doc = " for later use."]
#[doc = ""]
#[doc = " @param is_hard_cut True if the preset was switched using a hard cut via beat detection."]
#[doc = " @param index The playlist index of the new preset."]
#[doc = " @param message The error message."]
#[doc = " @param user_data A user-defined data pointer that was provided when registering the callback,"]
#[doc = "                  e.g. context information."]
pub type projectm_preset_switch_failed_event = ::std::option::Option<
    unsafe extern "C" fn(
        is_hard_cut: bool,
        index: u32,
        message: *const ::std::os::raw::c_char,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;
#[doc = " @brief Callback function that is executed if a preset rating has been changed."]
#[doc = ""]
#[doc = " Can be used for example to update the rating display in the host application."]
#[doc = ""]
#[doc = " @param index The playlist index of the new preset."]
#[doc = " @param rating The new rating value."]
#[doc = " @param rating_type The rating type that has been changed."]
#[doc = " @param user_data A user-defined data pointer that was provided when registering the callback,"]
#[doc = "                  e.g. context information."]
pub type projectm_preset_rating_changed_event = ::std::option::Option<
    unsafe extern "C" fn(
        index: u32,
        rating: i32,
        rating_type: projectm_preset_rating_type,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

pub fn projectm_create(setting_file_path: String, flags: i32) -> projectm_handle {
    extern "C" {
        #[doc = " @brief Creates a new projectM instance, reading settings from the given file."]
        #[doc = " @param setting_file_path A path to the settings file to read the configuration from."]
        #[doc = "                          If NULL or an empty path are provided, default settings will be used."]
        #[doc = " @param flags Any combination of values from the projectm_flags enumeration."]
        #[doc = " @return A projectM handle for the newly created instance that must be used in subsequent API calls."]
        #[doc = "         NULL if the instance could not be created successfully."]
        pub fn projectm_create(
            setting_file_path: *const ::std::os::raw::c_char,
            flags: i32,
        ) -> projectm_handle;
    }

    let setting_file_path_str = CString::new(setting_file_path).expect("CString::new failed");
    
    return unsafe {
        projectm_create(setting_file_path_str.as_ptr(), flags)
    }
}

pub fn projectm_create_settings(settings: *const projectm_settings, flags: i32) -> projectm_handle {
    extern "C" {
        #[doc = " @brief Creates a new projectM instance with given settings."]
        #[doc = " @param settings A pointer to a projectm_settings_t with the settings to be used by the new instance."]
        #[doc = "                 If this pointer is NULL, default settings will be used."]
        #[doc = " @param flags Any combination of values from the projectm_flags enumeration."]
        #[doc = " @return A projectM handle for the newly created instance that must be used in subsequent API calls."]
        #[doc = "         NULL if the instance could not be created successfully."]
        pub fn projectm_create_settings(
            settings: *const projectm_settings,
            flags: i32,
        ) -> projectm_handle;
    }

    return unsafe {
        projectm_create_settings(settings, flags)
    }
}

pub fn projectm_destroy(instance: projectm_handle) {
    extern "C" {
        #[doc = " @brief Destroys the given instance and frees the resources."]
        #[doc = ""]
        #[doc = " After destroying the handle, it must not be used for any other calls to the API."]
        #[doc = ""]
        #[doc = " @param instance A handle returned by projectm_create() or projectm_create_settings()."]
        pub fn projectm_destroy(instance: projectm_handle);
    }

    unsafe {
        projectm_destroy(instance)
    }
}

pub fn projectm_set_preset_switched_event_callback(instance: projectm_handle, callback: projectm_preset_switched_event, user_data: *mut ::std::os::raw::c_void) {
    extern "C" {
        #[doc = " @brief Sets a callback function that will be called when a preset changes."]
        #[doc = ""]
        #[doc = " Only one callback can be registered per projectM instance. To remove the callback, use NULL."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param callback A pointer to the callback function."]
        #[doc = " @param user_data A pointer to any data that will be sent back in the callback, e.g. context information."]
        pub fn projectm_set_preset_switched_event_callback(
            instance: projectm_handle,
            callback: projectm_preset_switched_event,
            user_data: *mut ::std::os::raw::c_void,
        );
    }

    unsafe {
        projectm_set_preset_switched_event_callback(instance, callback, user_data)
    }
}

pub fn projectm_set_shuffle_enable_changed_event_callback(instance: projectm_handle, callback: projectm_shuffle_enable_changed_event, user_data: *mut ::std::os::raw::c_void) {
    extern "C" {
        #[doc = " @brief Sets a callback function that will be called when the shuffle setting changes."]
        #[doc = ""]
        #[doc = " Only one callback can be registered per projectM instance. To remove the callback, use NULL."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param callback A pointer to the callback function."]
        #[doc = " @param user_data A pointer to any data that will be sent back in the callback, e.g. context information."]
        pub fn projectm_set_shuffle_enable_changed_event_callback(
            instance: projectm_handle,
            callback: projectm_shuffle_enable_changed_event,
            user_data: *mut ::std::os::raw::c_void,
        );
    }

    unsafe {
        projectm_set_shuffle_enable_changed_event_callback(instance, callback, user_data)
    }
}

pub fn projectm_set_preset_switch_failed_event_callback(instance: projectm_handle, callback: projectm_preset_switch_failed_event, user_data: *mut ::std::os::raw::c_void) {
    extern "C" {
        #[doc = " @brief Sets a callback function that will be called when a preset change failed."]
        #[doc = ""]
        #[doc = " Only one callback can be registered per projectM instance. To remove the callback, use NULL."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param callback A pointer to the callback function."]
        #[doc = " @param user_data A pointer to any data that will be sent back in the callback, e.g. context information."]
        pub fn projectm_set_preset_switch_failed_event_callback(
            instance: projectm_handle,
            callback: projectm_preset_switch_failed_event,
            user_data: *mut ::std::os::raw::c_void,
        );
    }

    unsafe {
        projectm_set_preset_switch_failed_event_callback(instance, callback, user_data)
    }
}

pub fn projectm_set_preset_rating_changed_event_callback(instance: projectm_handle, callback: projectm_preset_rating_changed_event, user_data: *mut ::std::os::raw::c_void) {
    extern "C" {
        #[doc = " @brief Sets a callback function that will be called when a preset rating changed."]
        #[doc = ""]
        #[doc = " Only one callback can be registered per projectM instance. To remove the callback, use NULL."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param callback A pointer to the callback function."]
        #[doc = " @param user_data A pointer to any data that will be sent back in the callback, e.g. context information."]
        pub fn projectm_set_preset_rating_changed_event_callback(
            instance: projectm_handle,
            callback: projectm_preset_rating_changed_event,
            user_data: *mut ::std::os::raw::c_void,
        );
    }

    unsafe {
        projectm_set_preset_rating_changed_event_callback(instance, callback, user_data)
    }
}

pub fn projectm_reset_gl(instance: projectm_handle) {
    extern "C" {
      #[doc = " @brief Reset the projectM OpenGL renderer."]
      #[doc = ""]
      #[doc = " <p>Required if anything invalidates the state of the current OpenGL context projectM is rendering to.</p>"]
      #[doc = ""]
      #[doc = " <p>For resize events, it is sufficient to call projectm_set_window_size()</p>"]
      #[doc = ""]
      #[doc = " @param instance The projectM instance handle."]
      pub fn projectm_reset_gl(instance: projectm_handle);
    }
  
    unsafe {
      projectm_reset_gl(instance);
    }
}

pub fn projectm_reset_textures(instance: projectm_handle) {
    extern "C" {
        #[doc = " @brief Reloads all textures."]
        #[doc = ""]
        #[doc = " Also resets the OpenGL renderer without changing the viewport size. Useful if preset paths were changed."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        pub fn projectm_reset_textures(instance: projectm_handle);
    }

    unsafe {
        projectm_reset_textures(instance)
    }
}

pub fn projectm_get_title(instance: projectm_handle) -> String {
    extern "C" {
      #[doc = " @brief Returns the current title text."]
      #[doc = " @param instance The projectM instance handle."]
      #[doc = " @return The currently set title text."]
      pub fn projectm_get_title(instance: projectm_handle) -> *const ::std::os::raw::c_char;
    }
    
    let title = unsafe { projectm_get_title(instance) };     
    
    let title_str = unsafe { CStr::from_ptr(title) };

    return title_str.to_str().unwrap().to_string();
}

pub fn projectm_set_title(instance: projectm_handle, title: String) {
    extern "C" {
      #[doc = " @brief Sets the current title text and displays it."]
      #[doc = " @param instance The projectM instance handle."]
      #[doc = " @param title The title text to display."]
      pub fn projectm_set_title(instance: projectm_handle, title: *const ::std::os::raw::c_char);
    }
  
    let title = CString::new(title).expect("CString::new failed");
  
    unsafe {
      projectm_set_title(instance, title.as_ptr())
    }
}

pub fn projectm_render_frame(instance: projectm_handle) {
    extern "C" {
        #[doc = " @brief Renders a single frame."]
        #[doc = ""]
        #[doc = " @note Separate two-pass frame rendering is currently not supported by the C API as it is rarely used"]
        #[doc = "       and also depends on the loaded preset."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        pub fn projectm_render_frame(instance: projectm_handle);
    }

    unsafe {
        projectm_render_frame(instance)
    }
}

pub fn projectm_init_render_to_texture(instance: projectm_handle) -> u32 {
    extern "C" {
        #[doc = " @brief Enables render-to-texture."]
        #[doc = ""]
        #[doc = " Useful if projectM output will be part of a more complex OpenGL scene. The size of the texture is determined by the"]
        #[doc = " given viewport size and the dimensions should be a power of 2."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @return A GLuint value with the texture ID projectM will render to."]
        pub fn projectm_init_render_to_texture(instance: projectm_handle) -> u32;
    }

    return unsafe {
        projectm_init_render_to_texture(instance)
    }
}

pub fn projectm_key_handler(instance: projectm_handle, event: projectMEvent, keycode: projectMKeycode, modifier: projectMModifier) {
    extern "C" {
        #[doc = " @brief Key handler that processes user input."]
        #[doc = ""]
        #[doc = " <p>This method can be used to send user input in the host application to projectM, for example"]
        #[doc = " to switch presets, display the help and search menus or change settings like beat sensitivity.</p>"]
        #[doc = ""]
        #[doc = " <p>All actions executed by the key handler can also be run programmatically if the host application"]
        #[doc = " is not able to redirect keyboard input to projectM.</p>"]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param event The key event, valid are either PROJECTM_KEYUP or PROJECTM_KEYDOWN."]
        #[doc = " @param keycode The key code, mapped to a value of the projectMKeycode enumeration."]
        #[doc = " @param modifier The key modifier as a value from the projectMModifier."]
        pub fn projectm_key_handler(
            instance: projectm_handle,
            event: projectMEvent,
            keycode: projectMKeycode,
            modifier: projectMModifier,
        );
    }

    unsafe {
        projectm_key_handler(instance, event, keycode, modifier)
    }
}

pub fn projectm_default_key_handler(instance: projectm_handle, event: projectMEvent, keycode: projectMKeycode) {
    extern "C" {
        #[doc = " @brief Default key handler that processes user input."]
        #[doc = ""]
        #[doc = " <p>This method can be used to send user input in the host application to projectM, for example"]
        #[doc = " to switch presets, display the help and search menus or change settings like beat sensitivity.</p>"]
        #[doc = ""]
        #[doc = " <p>All actions executed by the key handler can also be run programmatically if the host application"]
        #[doc = " is not able to redirect keyboard input to projectM.</p>"]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param event The key event, valid are either PROJECTM_KEYUP or PROJECTM_KEYDOWN."]
        #[doc = " @param keycode The key code, mapped to a value of the projectMKeycode enumeration."]
        pub fn projectm_default_key_handler(
            instance: projectm_handle,
            event: projectMEvent,
            keycode: projectMKeycode,
        );
    }

    unsafe {
        projectm_default_key_handler(instance, event, keycode)
    }
}

// TODO: STARTED HERE

pub fn projectm_get_texture_size(instance: projectm_handle) -> u32 {
    extern "C" {
        #[doc = " @brief Returns the size of the internal render texture."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @return The size of the internal rendering texture."]
        pub fn projectm_get_texture_size(instance: projectm_handle) -> u32;
    }

    return unsafe {
        projectm_get_texture_size(instance)
    }
}

pub fn projectm_set_texture_size(instance: projectm_handle, size: usize) {
    extern "C" {
        #[doc = " @brief Changes the size of the internal render texture."]
        #[doc = " @note This will recreate the internal renderer."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param size The new size of the render texture. Must be a power of 2."]
        pub fn projectm_set_texture_size(instance: projectm_handle, size: usize);
    }

    unsafe {
        projectm_set_texture_size(instance, size)
    }
}

pub fn projectm_get_hard_cut_duration(instance: projectm_handle) -> f64 {
    extern "C" {
        #[doc = " @brief Returns the minimum display time before a hard cut can happen."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @return The minimum number of seconds the preset will be displayed before a hard cut."]
        pub fn projectm_get_hard_cut_duration(instance: projectm_handle) -> f64;
    }

    return unsafe {
        projectm_get_hard_cut_duration(instance)
    }
}

pub fn projectm_set_hard_cut_duration(instance: projectm_handle, seconds: f64) {
    extern "C" {
        #[doc = " @brief Sets the minimum display time before a hard cut can happen."]
        #[doc = ""]
        #[doc = " <p>Hard cuts are beat-sensitive preset transitions, immediately changing from"]
        #[doc = " one preset to the next without a smooth blending period.</p>"]
        #[doc = ""]
        #[doc = " <p>Set this to a higher value than preset duration to disable hard cuts.</p>"]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param seconds Minimum number of seconds the preset will be displayed before a hard cut."]
        pub fn projectm_set_hard_cut_duration(instance: projectm_handle, seconds: f64);
    }

    unsafe {
        projectm_set_hard_cut_duration(instance, seconds)
    }
}

pub fn projectm_get_hard_cut_enabled(instance: projectm_handle) -> bool {
    extern "C" {
        #[doc = " @brief Returns whether hard cuts are enabled or not."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @return True if hard cuts are enabled, false otherwise."]
        pub fn projectm_get_hard_cut_enabled(instance: projectm_handle) -> bool;
    }

    return unsafe {
        projectm_get_hard_cut_enabled(instance)
    }
}

pub fn projectm_set_hard_cut_enabled(instance: projectm_handle, enabled: bool) {
    extern "C" {
        #[doc = " @brief Enables or disables hard cuts."]
        #[doc = ""]
        #[doc = " Even if enabled, the hard cut duration must be set to a value lower than the preset duration"]
        #[doc = " to work properly."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param enabled True to enable hard cuts, false to disable."]
        pub fn projectm_set_hard_cut_enabled(instance: projectm_handle, enabled: bool);
    }

    unsafe {
        projectm_set_hard_cut_enabled(instance, enabled)
    }
}

pub fn projectm_get_hard_cut_sensitivity(instance: projectm_handle) -> f32 {
    extern "C" {
        #[doc = " @brief Returns the current hard cut sensitivity."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @return The current hard cut sensitivity."]
        pub fn projectm_get_hard_cut_sensitivity(instance: projectm_handle) -> f32;
    }

    return unsafe {
        projectm_get_hard_cut_sensitivity(instance)
    }
}

pub fn projectm_set_hard_cut_sensitivity(instance: projectm_handle, sensitivity: f32) {
    extern "C" {
        #[doc = " @brief Sets the hard cut volume sensitivity."]
        #[doc = ""]
        #[doc = " The beat detection volume difference that must be surpassed to trigger a hard cut."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param sensitivity The volume threshold that triggers a hard cut if surpassed."]
        pub fn projectm_set_hard_cut_sensitivity(instance: projectm_handle, sensitivity: f32);
    }

    unsafe {
        projectm_set_hard_cut_sensitivity(instance, sensitivity)
    }
}

pub fn projectm_get_soft_cut_duration(instance: projectm_handle) -> f64 {
    extern "C" {
        #[doc = " @brief Returns the time in seconds for a soft transition between two presets."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @return Time in seconds it takes to smoothly transition from one preset to another."]
        pub fn projectm_get_soft_cut_duration(instance: projectm_handle) -> f64;
    }
    return unsafe {
        projectm_get_soft_cut_duration(instance)
    }
}

pub fn projectm_set_soft_cut_duration(instance: projectm_handle, seconds: f64) {
    extern "C" {
        #[doc = " @brief Sets the time in seconds for a soft transition between two presets."]
        #[doc = ""]
        #[doc = " During a soft cut, both presets are rendered and slowly transitioned from one"]
        #[doc = " to the other."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param seconds Time in seconds it takes to smoothly transition from one preset to another."]
        pub fn projectm_set_soft_cut_duration(instance: projectm_handle, seconds: f64);
    }

    unsafe {
        projectm_set_soft_cut_duration(instance, seconds)
    }
}

pub fn projectm_set_preset_duration(instance: projectm_handle, seconds: f64) {
    extern "C" {
        #[doc = " @brief Sets the preset display duration before switching to the next using a soft cut."]
        #[doc = ""]
        #[doc = " This can be considered as the maximum time a preset is displayed. If this time is reached,"]
        #[doc = " a smooth cut will be initiated. A hard cut, if any, will always happen before this time."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param seconds The number of seconds a preset will be displayed before the next is shown."]
        pub fn projectm_set_preset_duration(instance: projectm_handle, seconds: f64);
    }

    unsafe {
        projectm_set_preset_duration(instance, seconds)
    }
}

pub fn projectm_get_mesh_x(instance: projectm_handle) -> u32 {
    extern "C" {
        #[doc = " @brief Returns the per-pixel equation mesh size in units."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param width The width of the mesh."]
        #[doc = " @param height The height of the mesh."]
        pub fn projectm_get_mesh_size(
            instance: projectm_handle,
            width: *mut u32,
            height: *mut u32,
        );
    }
  
    #[derive(Debug, Default, Copy, Clone)]
    #[repr(C, packed)]
    struct Mesh {
        mesh_x: u32,
        mesh_y: u32,
    }
  
    let mut mesh = Mesh::default();
    
    unsafe {
        projectm_get_mesh_size(instance, std::ptr::addr_of_mut!(mesh.mesh_x), std::ptr::addr_of_mut!(mesh.mesh_y));
    }
  
  
    return mesh.mesh_x;
}

pub fn projectm_get_mesh_y(instance: projectm_handle) -> u32 {
    extern "C" {
        #[doc = " @brief Returns the per-pixel equation mesh size in units."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param width The width of the mesh."]
        #[doc = " @param height The height of the mesh."]
        pub fn projectm_get_mesh_size(
            instance: projectm_handle,
            width: *mut u32,
            height: *mut u32,
        );
    }

    #[derive(Debug, Default, Copy, Clone)]
    #[repr(C, packed)]
    struct Mesh {
        mesh_x: u32,
        mesh_y: u32,
    }

    let mut mesh = Mesh::default();

    unsafe {
        projectm_get_mesh_size(instance, std::ptr::addr_of_mut!(mesh.mesh_x), std::ptr::addr_of_mut!(mesh.mesh_y));
    }


    return mesh.mesh_y;
}

pub fn projectm_get_mesh_size(instance: projectm_handle) -> [u32;2] {
    extern "C" {
        #[doc = " @brief Returns the per-pixel equation mesh size in units."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param width The width of the mesh."]
        #[doc = " @param height The height of the mesh."]
        pub fn projectm_get_mesh_size(
            instance: projectm_handle,
            width: *mut u32,
            height: *mut u32,
        );
    }

    #[derive(Debug, Default, Copy, Clone)]
    #[repr(C, packed)]
    struct Mesh {
        mesh_x: u32,
        mesh_y: u32,
    }

    let mut mesh = Mesh::default();

    unsafe {
        projectm_get_mesh_size(instance, std::ptr::addr_of_mut!(mesh.mesh_x), std::ptr::addr_of_mut!(mesh.mesh_y));
    }

    return [ mesh.mesh_x, mesh.mesh_y ];
}

pub fn projectm_get_fps(instance: projectm_handle) -> u32 {
    extern "C" {
        #[doc = " @brief Returns the target frames per second count."]
        #[doc = " @note This is not the actual FPS, but the targeted refresh framerate if the integrating application."]
        #[doc = " @param instance The projectM instance handle."]
        pub fn projectm_get_fps(instance: projectm_handle) -> u32;
    }

    return unsafe {
        projectm_get_fps(instance)
    }
}

pub fn projectm_get_preset_path(instance: projectm_handle) -> str {
    extern "C" {
        #[doc = " @brief Returns the search path for presets and textures."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @return The path used to search for presets and textures."]
        pub fn projectm_get_preset_path(instance: projectm_handle) -> *const ::std::os::raw::c_char;
    }
  
    let preset_path = unsafe { projectm_get_preset_path(instance) };

    let preset_path_str = unsafe { CStr::from_ptr(preset_path) };
  
    return preset_path_str.to_str().unwrap();
}

pub fn projectm_get_title_font_filename(instance: projectm_handle) -> str {
    extern "C" {
        #[doc = " @brief Returns the path and filename of the font used to render the title overlay text."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @return The path and filename of the title text font."]
        pub fn projectm_get_title_font_filename(
            instance: projectm_handle,
        ) -> *const ::std::os::raw::c_char;
    }

    let title_font_filename = unsafe { projectm_get_title_font_filename(instance) };

    let title_font_filename_str = unsafe { CStr::from_ptr(title_font_filename) };

    return title_font_filename_str.to_str().unwrap();
}

pub fn projectm_get_menu_font_filename(instance: projectm_handle) -> str {
    extern "C" {
        #[doc = " @brief Returns the path and filename of the font used to render the menu overlay text."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @return The path and filename of the menu text font."]
        pub fn projectm_get_menu_font_filename(
            instance: projectm_handle,
        ) -> *const ::std::os::raw::c_char;
    }

    let menu_font_filename = unsafe { projectm_get_menu_font_filename(instance) };

    let menu_font_filename_str = unsafe { CStr::from_ptr(menu_font_filename) };

    return menu_font_filename_str.to_str().unwrap();
}

pub fn projectm_get_data_dir_path(instance: projectm_handle) -> str {
    extern "C" {
        #[doc = " @brief Returns the path projectM uses to search for additional data."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @return The data dir path."]
        pub fn projectm_get_data_dir_path(instance: projectm_handle) -> *const ::std::os::raw::c_char;
    }

    let data_dir_path = unsafe { projectm_get_data_dir_path(instance) };

    let data_dir_path_str = unsafe { CStr::from_ptr(data_dir_path) };

    return data_dir_path_str.to_str().unwrap();
}

pub fn projectm_set_aspect_correction(instance: projectm_handle, enabled: bool) {
    extern "C" {
        #[doc = " @brief Enabled or disables aspect ratio correction in presets that support it."]
        #[doc = ""]
        #[doc = " This sets a flag presets can use to aspect-correct rendered shapes, which otherwise would"]
        #[doc = " be distorted if the viewport isn't exactly square."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param enabled True to enable aspect correction, false to disable it."]
        pub fn projectm_set_aspect_correction(instance: projectm_handle, enabled: bool);
    }
  
    unsafe { projectm_set_aspect_correction(instance, enabled)
    }
}

pub fn projectm_get_aspect_correction(instance: projectm_handle) -> bool {
    extern "C" {
        #[doc = " @brief Returns whether aspect ratio correction is enabled or not."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @return True if aspect ratio correction is enabled, false otherwise."]
        pub fn projectm_get_aspect_correction(instance: projectm_handle) -> bool;
    }
    
    return unsafe { projectm_get_aspect_correction(instance) };
}

pub fn projectm_set_easter_egg(instance: projectm_handle, value: f32) {
    extern "C" {
        #[doc = " @brief Sets the \"easter egg\" value."]
        #[doc = ""]
        #[doc = " <p>This doesn't enable any fancy feature, it only influences the randomized display time of presets. It's"]
        #[doc = " passed as the \"sigma\" value of the gaussian random number generator used to determine the maximum display time,"]
        #[doc = " effectively multiplying the generated number of seconds by this amount.</p>"]
        #[doc = ""]
        #[doc = " <p>See function sampledPresetDuration() of the TimeKeeper class on how it is used.</p>"]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param value The new \"easter egg\" value."]
        pub fn projectm_set_easter_egg(instance: projectm_handle, value: f32);
    }
  
    unsafe { 
        projectm_set_easter_egg(instance, value) 
    };
}

pub fn projectm_get_easter_egg(instance: projectm_handle) -> f32 {
    extern "C" {
        #[doc = " @brief Returns the current \"easter egg\" value."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @return The current \"easter egg\" value."]
        pub fn projectm_get_easter_egg(instance: projectm_handle) -> f32;
    }
    
    return unsafe { 
        projectm_get_easter_egg(instance) 
    };
}

pub fn projectm_touch(instance: projectm_handle, x: f32, y: f32, pressure: i32, touch_type: projectm_touch_type) {
    extern "C" {
        #[doc = " @brief Starts a touch event or moves an existing waveform."]
        #[doc = ""]
        #[doc = " This will add or move waveforms in addition to the preset waveforms. If there is an existing waveform"]
        #[doc = " at the given coordinates, it will be centered on the new coordinates. If there is no waveform, a new one"]
        #[doc = " will be added."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param x The x coordinate of the touch event."]
        #[doc = " @param y The y coordinate of the touch event."]
        #[doc = " @param pressure  The amount of pressure applied in a range from 0.0 to 1.0."]
        #[doc = " @param touch_type The waveform type that will be rendered on touch."]
        pub fn projectm_touch(
            instance: projectm_handle,
            x: f32,
            y: f32,
            pressure: i32,
            touch_type: projectm_touch_type,
        );
    }

    unsafe {
        projectm_touch(instance, x, y, pressure, touch_type)
    }
}

pub fn projectm_touch_drag(instance: projectm_handle, x: f32, y: f32, pressure: i32) {
    extern "C" {
        #[doc = " @brief Centers any waveforms under the coordinates to simulate dragging."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param x The x coordinate of the drag."]
        #[doc = " @param y the y coordinate of the drag."]
        #[doc = " @param pressure The amount of pressure applied in a range from 0.0 to 1.0."]
        pub fn projectm_touch_drag(
            instance: projectm_handle,
            x: f32,
            y: f32,
            pressure: i32,
        );
    }

    unsafe {
        projectm_touch_drag(instance, x, y, pressure)
    }
}

pub fn projectm_touch_destroy(instance: projectm_handle, x: f32, y: f32) {
    extern "C" {
        #[doc = " @brief Removes any additional touch waveforms under the given coordinates."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param x The last known x touch coordinate."]
        #[doc = " @param y The last known y touch coordinate."]
        pub fn projectm_touch_destroy(instance: projectm_handle, x: f32, y: f32);
    }

    unsafe {
        projectm_touch_destroy(instance, x, y)
    }
}

pub fn projectm_touch_destroy_all(instance: projectm_handle) {
    extern "C" {
        #[doc = " @brief Removes all touch waveforms from the screen."]
        #[doc = ""]
        #[doc = " Preset-defined waveforms will still be displayed."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        pub fn projectm_touch_destroy_all(instance: projectm_handle);
    }

    unsafe {
        projectm_touch_destroy_all(instance)
    }
}

//  !FIX convert string
pub fn projectm_set_help_text(instance: projectm_handle, help_text: String) {
    extern "C" {
        #[doc = " @brief Sets the help menu text."]
        #[doc = ""]
        #[doc = " The help menu will be toggled if the key mapped to PROJECTM_K_F1 is pressed."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param help_text The help text to be displayed."]
        pub fn projectm_set_help_text(
            instance: projectm_handle,
            help_text: *const ::std::os::raw::c_char,
        );
    }

    let help_text_str = CString::new(help_text).expect("CString::new failed");

    unsafe {
        projectm_set_help_text(instance, help_text_str.as_ptr())
    }
}

//  !FIX convert string
pub fn projectm_set_toast_message(instance: projectm_handle, toast_message: String) {
    extern "C" {
        #[doc = " @brief Displays a short message in the center of the rendering area for a few seconds."]
        #[doc = ""]
        #[doc = " <p>Useful to display song titles and changed audio settings. Used internally by projectM to display setting"]
        #[doc = " changes like preset lock.</p>"]
        #[doc = ""]
        #[doc = " <p>Only one toast message is shown at a time. If this method is called while another message is shown, it"]
        #[doc = " will be replaced immediately.</p>"]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param toast_message The message to display."]
        pub fn projectm_set_toast_message(
            instance: projectm_handle,
            toast_message: *const ::std::os::raw::c_char,
        );
    }

    let toast_message_str = CString::new(toast_message).expect("CString::new failed");

    unsafe {
        projectm_set_toast_message(instance, toast_message_str.as_ptr())
    }
}

pub fn projectm_get_settings(instance: projectm_handle) -> *mut projectm_settings {
    extern "C" {
        #[doc = " @brief Returns a structure with the current projectM settings."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @return A struct with all currently used settings."]
        pub fn projectm_get_settings(instance: projectm_handle) -> *mut projectm_settings;
    }

    return unsafe { projectm_get_settings(instance) };
}

// !FIX convert path to c_char
pub fn projectm_write_config(config_file: String, settings: *const projectm_settings) {
    extern "C" {
        #[doc = " @brief Saves the given settings struct into a file."]
        #[doc = ""]
        #[doc = " The file can be loaded during projectM initialization. This is useful if the application needs to"]
        #[doc = " keep settings separate from the global system/user configuration."]
        #[doc = ""]
        #[doc = " @param config_file The filename to store the settings in."]
        #[doc = " @param settings The settings struct to store."]
        pub fn projectm_write_config(
            config_file: *const ::std::os::raw::c_char,
            settings: *const projectm_settings,
        );
    }

    let config_file_str = CString::new(config_file).expect("CString::new failed");

    unsafe {
        projectm_write_config(config_file_str.as_ptr(), settings)
    }
}

//------------------------------------------------------------------------ 

pub fn projectm_select_preset_position(instance: projectm_handle, index: u32) {
    extern "C" {
        #[doc = " @brief Selects a preset, but does not display it."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param index The preset index to select."]
        pub fn projectm_select_preset_position(
            instance: projectm_handle,
            index: u32,
        );
    }

    unsafe {
        projectm_select_preset_position(instance, index)
    }
}

pub fn projectm_select_preset(instance: projectm_handle, index: u32, hard_cut: bool) {
    extern "C" {
        #[doc = " @brief Selects and displays the preset."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param index the preset to display."]
        #[doc = " @param hard_cut If true, a hard cut is made, otherwise it will be blended smoothly."]
        pub fn projectm_select_preset(
            instance: projectm_handle,
            index: u32,
            hard_cut: bool,
        );
    }

    unsafe {
        projectm_select_preset(instance, index, hard_cut)
    }
}

pub fn projectm_populate_preset_menu(instance: projectm_handle) {
    extern "C" {
        #[doc = " @brief Populates the on-screen preset menu."]
        #[doc = " @param instance The projectM instance handle."]
        pub fn projectm_populate_preset_menu(instance: projectm_handle);
    }

    unsafe {
        projectm_populate_preset_menu(instance)
    }
}

pub fn projectm_remove_preset(instance: projectm_handle, index: u32) {
    extern "C" {
        #[doc = " @brief Removes a preset from the playlist."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param index The  preset index to remove from the playlist."]
        pub fn projectm_remove_preset(instance: projectm_handle, index: u32);
    }

    unsafe {
        projectm_remove_preset(instance, index)
    }
}

pub fn projectm_clear_playlist(instance: projectm_handle) {
    extern "C" {
        #[doc = " @brief Clears the preset playlist."]
        #[doc = " @param instance The projectM instance handle."]
        pub fn projectm_clear_playlist(instance: projectm_handle);
    }

    unsafe {
        projectm_clear_playlist(instance)
    }
}

pub fn projectm_lock_preset(instance: projectm_handle, lock: bool) {
    extern "C" {
        #[doc = " @brief Locks or unlocks the current preset."]
        #[doc = ""]
        #[doc = " Locking effectively disables automatic preset transitions, both hard and soft cuts. Programmatic"]
        #[doc = " preset switches will still be executed."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param lock True to lock the current preset, false to enable automatic transitions."]
        pub fn projectm_lock_preset(instance: projectm_handle, lock: bool);
    }

    unsafe {
        projectm_lock_preset(instance, lock)
    }
}

pub fn projectm_is_preset_locked(instance: projectm_handle) -> bool {
    extern "C" {
        #[doc = " @brief Returns whether the current preset is locked or not."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @return True if the preset lock is enabled, false otherwise."]
        pub fn projectm_is_preset_locked(instance: projectm_handle) -> bool;
    }

    return unsafe {
        projectm_is_preset_locked(instance)
    }
}

pub fn projectm_is_text_input_active(instance: projectm_handle, no_minimum_length: bool) -> bool {
    extern "C" {
        #[doc = " @brief Returns whether the search text input mode is active or not."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param no_minimum_length If set to true, will return true if at least one character has been typed, otherwise"]
        #[doc = "                          a minimum length of three characters is required."]
        #[doc = " @return True if text input mode is active, false otherwise."]
        pub fn projectm_is_text_input_active(
            instance: projectm_handle,
            no_minimum_length: bool,
        ) -> bool;
    }

    return unsafe {
        projectm_is_text_input_active(instance, no_minimum_length)
    }
}

pub fn projectm_get_preset_index(instance: projectm_handle, preset_name: String) -> u32 {
    extern "C" {
        #[doc = " @brief Returns the playlist index for the given preset name."]
        #[doc = ""]
        #[doc = " If the preset name is found multiple times, the first matching index will be returned."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param preset_name The preset name to search for."]
        #[doc = " @return The first found playlist index of the requested preset, or 0 if the preset wasn't found."]
        pub fn projectm_get_preset_index(
            instance: projectm_handle,
            preset_name: *const ::std::os::raw::c_char,
        ) -> u32;
    }

    let preset_name_str = CString::new(preset_name).expect("CString::new failed");

    return unsafe {
        projectm_get_preset_index(instance, preset_name_str.as_ptr())
    }
}

pub fn projectm_select_preset_by_name(instance: projectm_handle, preset_name: String, hard_cut: bool) {
    extern "C" {
        #[doc = " @brief Displays the preset with the given name."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param preset_name The preset name to search for."]
        #[doc = " @param hard_cut If true, the preset will be shown immediately, if false a soft transition will be rendered."]
        pub fn projectm_select_preset_by_name(
            instance: projectm_handle,
            preset_name: *const ::std::os::raw::c_char,
            hard_cut: bool,
        );
    }

    let preset_name_str = CString::new(preset_name).expect("CString::new failed");

    unsafe {
        projectm_select_preset_by_name(instance, preset_name_str.as_ptr(), hard_cut)
    }
}

// !ASK ask abou this fuction
// pub fn projectm_get_search_text(instance: projectm_handle) -> *const ::std::os::raw::c_char {
//     extern "C" {
//         #[doc = " @brief Returns the current preset search text."]
//         #[doc = " @param instance The projectM instance handle."]
//         #[doc = " @return The current search text used to search for presets in the playlist."]
//         pub fn projectm_get_search_text(instance: projectm_handle) -> *const ::std::os::raw::c_char;
//     }

//     let preset_name_str = CString::new(preset_name).expect("CString::new failed");

//     unsafe {
//         projectm_select_preset_by_name(instance, preset_name_str.as_ptr(), hard_cut)
//     }
// }

// extern "C" {
//     #[doc = " @brief Sets the current preset search text."]
//     #[doc = " @param instance The projectM instance handle."]
//     #[doc = " @param search_text The search text used to search for presets in the current playlist."]
//     pub fn projectm_set_search_text(
//         instance: projectm_handle,
//         search_text: *const ::std::os::raw::c_char,
//     );
// }
// extern "C" {
//     #[doc = " @brief Deletes one character from the preset search text."]
//     #[doc = ""]
//     #[doc = " This is equivalent to pressing DEL in a text box."]
//     #[doc = ""]
//     #[doc = " @param instance The projectM instance handle."]
//     pub fn projectm_delete_search_text(instance: projectm_handle);
// }
// extern "C" {
//     #[doc = " @brief Deletes the whole search text."]
//     #[doc = ""]
//     #[doc = " This will effectively leave preset search mode."]
//     #[doc = ""]
//     #[doc = " @param instance The projectM instance handle."]
//     pub fn projectm_reset_search_text(instance: projectm_handle);
// }

pub fn projectm_get_selected_preset_index(instance: projectm_handle, index: *mut u32) -> bool {
    extern "C" {
        #[doc = " @brief Returns the currently selected preset index."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param index A valid pointer to an unsigned int that will receive the preset index."]
        #[doc = " @return True if a preset idnex was returned, false if no preset was selected, e.g. the playlist is empty."]
        pub fn projectm_get_selected_preset_index(
            instance: projectm_handle,
            index: *mut u32,
        ) -> bool;
    }

    return unsafe {
        projectm_get_selected_preset_index(instance, index)
    }
}

pub fn projectm_add_preset_url(instance: projectm_handle, preset_url: String, preset_name: String, rating_list: *mut i32, rating_list_length: u32) {
    extern "C" {
        #[doc = " @brief Adds a new preset at the end of the playlist."]
        #[doc = ""]
        #[doc = " The rating list is one rating per value of the projectm_preset_rating_type enumeration, with each actual enum"]
        #[doc = " value used as the index. If the rating list has the wrong length, the preset will not be added."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param preset_url The full path and filename of the preset."]
        #[doc = " @param preset_name The display name of the preset."]
        #[doc = " @param rating_list A list with ratings for the preset, one rating per rating type."]
        #[doc = " @param rating_list_length Length of the preset rating list."]
        pub fn projectm_add_preset_url(
            instance: projectm_handle,
            preset_url: *const ::std::os::raw::c_char,
            preset_name: *const ::std::os::raw::c_char,
            rating_list: *mut i32,
            rating_list_length: u32,
        );
    }

    let preset_url_str = CString::new(preset_url).expect("CString::new failed");
    let preset_name_str = CString::new(preset_name).expect("CString::new failed");

    unsafe {
        projectm_add_preset_url(instance, preset_url_str.as_ptr(), preset_name_str.as_ptr(), rating_list, rating_list_length)
    }
}
pub fn projectm_insert_preset_url(instance: projectm_handle, index: u32, preset_url: String, preset_name: String, rating_list: *mut i32, rating_list_length: u32) {
    extern "C" {
        #[doc = " @brief Adds a new preset at the given position in the playlist."]
        #[doc = ""]
        #[doc = " The rating list is one rating per value of the projectm_preset_rating_type enumeration, with each actual enum"]
        #[doc = " value used as the index. If the rating list has the wrong length, the preset will not be added."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param index The playlist index to insert the preset at. Must be less than or equal to the length of"]
        #[doc = "              the playlist."]
        #[doc = " @param preset_url The full path and filename of the preset."]
        #[doc = " @param preset_name The display name of the preset."]
        #[doc = " @param rating_list A list with ratings for the preset, one rating per rating type."]
        #[doc = " @param rating_list_length Length of the preset rating list."]
        pub fn projectm_insert_preset_url(
            instance: projectm_handle,
            index: u32,
            preset_url: *const ::std::os::raw::c_char,
            preset_name: *const ::std::os::raw::c_char,
            rating_list: *mut i32,
            rating_list_length: u32,
        );
    }

    let preset_url_str = CString::new(preset_url).expect("CString::new failed");
    let preset_name_str = CString::new(preset_name).expect("CString::new failed");

    unsafe {
        projectm_insert_preset_url(instance, index, preset_url_str.as_ptr(), preset_name_str.as_ptr(), rating_list, rating_list_length)
    }
}

pub fn projectm_preset_position_valid(instance: projectm_handle) -> bool {
    extern "C" {
        #[doc = " @brief Returns whether the currently selected preset has a valid position in the playlist."]
        #[doc = ""]
        #[doc = " This function is useful to check if the currently displayed preset is still inside the bounds of"]
        #[doc = " the current playlist, for example after the playlist was changed."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @return True if the position is valid, false if outside bounds."]
        pub fn projectm_preset_position_valid(instance: projectm_handle) -> bool;
    }

    return unsafe {
        projectm_preset_position_valid(instance)
    }
}

pub fn projectm_get_preset_filename(instance: projectm_handle, index: u32) -> String {
    extern "C" {
        #[doc = " @brief Returns the path and filename of the preset at the requested playlist index."]
        #[doc = " @note Make sure the index is inside the playlist bounds!"]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param index The playlist index to return the filename for."]
        #[doc = " @return The full path and filename of the preset at the given index."]
        pub fn projectm_get_preset_filename(
            instance: projectm_handle,
            index: u32,
        ) -> *const ::std::os::raw::c_char;
    }

    let preset_filename = unsafe { projectm_get_preset_filename(instance, index) };

    let preset_filename_str = unsafe { CStr::from_ptr(preset_filename) };

    return preset_filename_str.to_str().unwrap().to_string();
}

pub fn projectm_get_preset_name(instance: projectm_handle, index: u32) -> String {
    extern "C" {
        #[doc = " @brief Returns the display name of the preset at the requested playlist index."]
        #[doc = " @note Make sure the index is inside the playlist bounds!"]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param index The playlist index to return the display name for."]
        #[doc = " @return The display name of the preset at the given index."]
        pub fn projectm_get_preset_name(
            instance: projectm_handle,
            index: u32,
        ) -> *const ::std::os::raw::c_char;
    }

    let preset_name = unsafe { projectm_get_preset_name(instance, index) };

    let preset_name_str = unsafe { CStr::from_ptr(preset_name) };

    return preset_name_str.to_str().unwrap().to_string();
}

pub fn projectm_set_preset_name(instance: projectm_handle, index: u32, name: String) {
    extern "C" {
        #[doc = " @brief Changes the display name of the given preset in the playlist."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param index the playlist item index to change."]
        #[doc = " @param name The new display name."]
        pub fn projectm_set_preset_name(
            instance: projectm_handle,
            index: u32,
            name: *const ::std::os::raw::c_char,
        );
    }

    let name_str = CString::new(name).expect("CString::new failed");
    
    unsafe { 
        projectm_set_preset_name(instance, index, name_str.as_ptr()) 
    };
}

pub fn projectm_get_preset_rating(instance: projectm_handle, index: u32, rating_type: projectm_preset_rating_type) -> i32 {
    extern "C" {
        #[doc = " @brief Returns the rating for the given index and transition type."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param index The playlist item to retrieve the rating from."]
        #[doc = " @param rating_type The rating type to retrieve, either hard or soft cut."]
        #[doc = " @return The rating value of the requested item and type."]
        pub fn projectm_get_preset_rating(
            instance: projectm_handle,
            index: u32,
            rating_type: projectm_preset_rating_type,
        ) -> i32;
    }

    return unsafe { 
        projectm_get_preset_rating(instance, index, rating_type) 
    };
}

pub fn projectm_set_preset_rating(instance: projectm_handle, index: u32, rating: i32, rating_type: projectm_preset_rating_type) {
    extern "C" {
        #[doc = " @brief Changes the rating or a playlist item and type."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param index the playlist item to change the rating of."]
        #[doc = " @param rating The new rating value."]
        #[doc = " @param rating_type The type of the rating, either hard or soft cut."]
        pub fn projectm_set_preset_rating(
            instance: projectm_handle,
            index: u32,
            rating: i32,
            rating_type: projectm_preset_rating_type,
        );
    }

    unsafe { 
        projectm_set_preset_rating(instance, index, rating, rating_type) 
    };
}

pub fn projectm_get_playlist_size(instance: projectm_handle) -> u32 {
    extern "C" {
        #[doc = " @brief Returns the number of presets in the current playlist."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @return The number of presets in the currently loaded playlist."]
        pub fn projectm_get_playlist_size(instance: projectm_handle) -> u32;
    }

    return unsafe { 
        projectm_get_playlist_size(instance) 
    };
}

pub fn projectm_get_shuffle_enabled(instance: projectm_handle) -> bool {
    extern "C" {
        #[doc = " @brief Returns whether playlist shuffling is currently enabled or not."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @return True if shuffle is enabled, false if not."]
        pub fn projectm_get_shuffle_enabled(instance: projectm_handle) -> bool;
    }

    return unsafe { 
        projectm_get_shuffle_enabled(instance) 
    };
}

pub fn projectm_set_shuffle_enabled(instance: projectm_handle, shuffle_enabled: bool) {
    extern "C" {
        #[doc = " @brief Enables or disables preset playlist shuffling."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param shuffle_enabled True to randomly select the next preset, false to skip to the next item in line."]
        pub fn projectm_set_shuffle_enabled(instance: projectm_handle, shuffle_enabled: bool);
    }

    return unsafe { 
        projectm_set_shuffle_enabled(instance, shuffle_enabled) 
    };
}
pub fn projectm_get_search_index(instance: projectm_handle, name: String) -> u32 {
    extern "C" {
        #[doc = " @brief Gets the index of the provided preset name in the current search result list."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param name The name of the preset to return the index for."]
        #[doc = " @return The search result list index of the given preset name."]
        pub fn projectm_get_search_index(
            instance: projectm_handle,
            name: *const ::std::os::raw::c_char,
        ) -> u32;
    }

    let name_str = CString::new(name).expect("CString::new failed");
        
    return unsafe { 
        projectm_get_search_index(instance, name_str.as_ptr()) 
    };
}

pub fn projectm_select_previous_preset(instance: projectm_handle, hard_cut: bool) {
    extern "C" {
        #[doc = " @brief Switches to the previous preset in the current playlist."]
        #[doc = ""]
        #[doc = " This is unaffected by the shuffle mode and will always switch to the previous item."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param hard_cut True to immediately perform to the previous preset, false to do a soft transition."]
        pub fn projectm_select_previous_preset(instance: projectm_handle, hard_cut: bool);
    }
            
    unsafe { 
        projectm_select_previous_preset(instance, hard_cut) 
    };
}
pub fn projectm_select_next_preset(instance: projectm_handle, hard_cut: bool) {
    extern "C" {
        #[doc = " @brief Switches to the next preset in the current playlist."]
        #[doc = ""]
        #[doc = " This is unaffected by the shuffle mode and will always switch to the next item."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param hard_cut True to immediately perform to the next preset, false to do a soft transition."]
        pub fn projectm_select_next_preset(instance: projectm_handle, hard_cut: bool);
    }
                
    unsafe { 
        projectm_select_next_preset(instance, hard_cut) 
    };
}

pub fn projectm_select_random_preset(instance: projectm_handle, hard_cut: bool) {
    extern "C" {
        #[doc = " @brief Switches to a random preset in the current playlist."]
        #[doc = ""]
        #[doc = " This is unaffected by the shuffle mode and will always switch to a random item."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param hard_cut True to immediately perform to a random preset, false to do a soft transition."]
        pub fn projectm_select_random_preset(instance: projectm_handle, hard_cut: bool);
    }
                    
    unsafe { 
        projectm_select_random_preset(instance, hard_cut) 
    };
}

pub fn projectm_get_window_width(instance: projectm_handle) -> u32 {
    extern "C" {
        #[doc = " @brief Returns the current viewport size in pixels."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param width Valid pointer to a usize variable that will receive the window width value."]
        #[doc = " @param height Valid pointer to a usize variable that will receive the window height value."]
        pub fn projectm_get_window_size(
            instance: projectm_handle,
            width: *mut u32,
            height: *mut u32,
        );
    }

    #[derive(Debug, Default, Copy, Clone)]
    #[repr(C, packed)]
    struct Window {
        width: u32,
        height: u32,
    }

    let mut window = Window::default();

    unsafe {
        projectm_get_window_size(instance, std::ptr::addr_of_mut!(window.width), std::ptr::addr_of_mut!(window.height))
    }

    return window.width;
}

pub fn projectm_get_window_height(instance: projectm_handle) -> u32 {
    extern "C" {
        #[doc = " @brief Returns the current viewport size in pixels."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param width Valid pointer to a usize variable that will receive the window width value."]
        #[doc = " @param height Valid pointer to a usize variable that will receive the window height value."]
        pub fn projectm_get_window_size(
            instance: projectm_handle,
            width: *mut u32,
            height: *mut u32,
        );
    }

    #[derive(Debug, Default, Copy, Clone)]
    #[repr(C, packed)]
    struct Window {
        width: u32,
        height: u32,
    }

    let mut window = Window::default();

    unsafe {
        projectm_get_window_size(instance, std::ptr::addr_of_mut!(window.width), std::ptr::addr_of_mut!(window.height))
    }

    return window.height;
}

pub fn projectm_get_window_size(instance: projectm_handle) -> [u32;2] {
    extern "C" {
        #[doc = " @brief Returns the current viewport size in pixels."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param width Valid pointer to a usize variable that will receive the window width value."]
        #[doc = " @param height Valid pointer to a usize variable that will receive the window height value."]
        pub fn projectm_get_window_size(
            instance: projectm_handle,
            width: *mut u32,
            height: *mut u32,
        );
    }

    #[derive(Debug, Default, Copy, Clone)]
    #[repr(C, packed)]
    struct Window {
        width: u32,
        height: u32,
    }

    let mut window = Window::default();

    unsafe {
        projectm_get_window_size(instance, std::ptr::addr_of_mut!(window.width), std::ptr::addr_of_mut!(window.height))
    }

    return [ window.width, window.height ];
}

pub fn projectm_set_window_width(instance: projectm_handle, width: u32){
    extern "C" {
        #[doc = " @brief Sets the current viewport size in pixels."]
        #[doc = ""]
        #[doc = " Calling this function will reset the OpenGL renderer."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param width New viewport width in pixels."]
        #[doc = " @param height New viewport height in pixels."]
        pub fn projectm_set_window_size(instance: projectm_handle, width: u32, height: u32);
    }

    let height = projectm_get_window_height(instance);

    unsafe {
        projectm_set_window_size(instance, width, height)
    }
}

pub fn projectm_set_window_height(instance: projectm_handle, height: u32){
    extern "C" {
        #[doc = " @brief Sets the current viewport size in pixels."]
        #[doc = ""]
        #[doc = " Calling this function will reset the OpenGL renderer."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param width New viewport width in pixels."]
        #[doc = " @param height New viewport height in pixels."]
        pub fn projectm_set_window_size(instance: projectm_handle, width: u32, height: u32);
    }

    let width = projectm_get_window_height(instance);

    unsafe {
        projectm_set_window_size(instance, width, height)
    }
}

pub fn projectm_set_window_size(instance: projectm_handle, width: u32, height: u32){
    extern "C" {
        #[doc = " @brief Sets the current viewport size in pixels."]
        #[doc = ""]
        #[doc = " Calling this function will reset the OpenGL renderer."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param width New viewport width in pixels."]
        #[doc = " @param height New viewport height in pixels."]
        pub fn projectm_set_window_size(instance: projectm_handle, width: u32, height: u32);
    }

    unsafe {
        projectm_set_window_size(instance, width, height)
    }
}

pub fn projectm_get_error_loading_current_preset(instance: projectm_handle) -> bool {
    extern "C" {
        #[doc = " @brief Returns whether the current preset was loaded successfully or not."]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @return True if the preset was not loaded successfully, false if it is displayed correctly."]
        pub fn projectm_get_error_loading_current_preset(instance: projectm_handle) -> bool;
    }

    return unsafe {
        projectm_get_error_loading_current_preset(instance)
    }

}

pub fn projectm_pcm_get_max_samples() -> u32 {
    extern "C" {
        #[doc = " @brief Returns the maximum number of audio samples that can be stored."]
        #[doc = ""]
        #[doc = " Each PCM data update should not exceed this number of samples. If more samples"]
        #[doc = " are added, only this number of samples is stored and the remainder discarded."]
        #[doc = ""]
        #[doc = " @return The number of audio samples that are stored, per channel."]
        pub fn projectm_pcm_get_max_samples() -> u32;
    }

    return unsafe {
        projectm_pcm_get_max_samples()
    }
}

pub fn projectm_pcm_add_float(instance: projectm_handle, samples: *const f32, count: u32, channels: projectm_channels) {
    extern "C" {
        #[doc = " @brief Adds 32-bit floating-point audio samples."]
        #[doc = ""]
        #[doc = " This function is used to add new audio data to projectM's internal audio buffer. It is internally converted"]
        #[doc = " to 2-channel float data, duplicating the channel."]
        #[doc = ""]
        #[doc = " If stereo, the channel order in samples is LRLRLR."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param samples An array of PCM samples."]
        #[doc = " Each sample is expected to be within the range -1 to 1."]
        #[doc = " @param count The number of audio samples in a channel."]
        #[doc = " @param channels If the buffer is mono or stereo."]
        #[doc = " Can be PROJECTM_MONO or PROJECTM_STEREO."]
        pub fn projectm_pcm_add_float(
            instance: projectm_handle,
            samples: *const f32,
            count: u32,
            channels: projectm_channels,
        );
    }

    unsafe {
        projectm_pcm_add_float(instance, samples, count, channels)
    }
}

pub fn projectm_pcm_add_int16(instance: projectm_handle, samples: *const i16, count: u32, channels: projectm_channels) {
    extern "C" {
        #[doc = " @brief Adds 16-bit integer audio samples."]
        #[doc = ""]
        #[doc = " This function is used to add new audio data to projectM's internal audio buffer. It is internally converted"]
        #[doc = " to 2-channel float data, duplicating the channel."]
        #[doc = ""]
        #[doc = " If stereo, the channel order in samples is LRLRLR."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param samples An array of PCM samples."]
        #[doc = " @param count The number of audio samples in a channel."]
        #[doc = " @param channels If the buffer is mono or stereo."]
        #[doc = " Can be PROJECTM_MONO or PROJECTM_STEREO."]
        pub fn projectm_pcm_add_int16(
            instance: projectm_handle,
            samples: *const i16,
            count: u32,
            channels: projectm_channels,
        );
    }

    unsafe {
        projectm_pcm_add_int16(instance, samples, count, channels)
    }
}

pub fn projectm_pcm_add_uint8(instance: projectm_handle, samples: *const u8, count: u32, channels: projectm_channels) {
    extern "C" {
        #[doc = " @brief Adds 8-bit unsigned integer audio samples."]
        #[doc = ""]
        #[doc = " This function is used to add new audio data to projectM's internal audio buffer. It is internally converted"]
        #[doc = " to 2-channel float data, duplicating the channel."]
        #[doc = ""]
        #[doc = " If stereo, the channel order in samples is LRLRLR."]
        #[doc = ""]
        #[doc = " @param instance The projectM instance handle."]
        #[doc = " @param samples An array of PCM samples."]
        #[doc = " @param count The number of audio samples in a channel."]
        #[doc = " @param channels If the buffer is mono or stereo."]
        #[doc = " Can be PROJECTM_MONO or PROJECTM_STEREO."]
        pub fn projectm_pcm_add_uint8(
            instance: projectm_handle,
            samples: *const u8,
            count: u32,
            channels: projectm_channels,
        );
    }

    unsafe {
        projectm_pcm_add_uint8(instance, samples, count, channels)
    }
}