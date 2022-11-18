use std::{thread, time};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use projectm_rs::*;

fn main() -> Result<(), String> {
    // setup sdl
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // create window
    let window = video_subsystem.window("projectm-rs-test-sdl", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    
    // create canvas/renderer
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    // projectm::settings
    let settings = settings {
        mesh_x: 96,
        mesh_y: 54,
        fps: 30,
        texture_size: 512,
        window_width: 1280,
        window_height: 720,
        preset_duration: 15.0,
        soft_cut_duration: 15.0,
        hard_cut_duration: 60.0,
        hard_cut_enabled: false,
        hard_cut_sensitivity: 0.0,
        beat_sensitivity: 0.5,
        aspect_correction: true,
        easter_egg: 0.5,
        shuffle_enabled: true,
        soft_cut_ratings_enabled: true,
        preset_path: String::from("./presets"),
        texture_path: String::from("./textures"),
        data_path: String::from("./"),
    };

    // projectm::init
    let projectm_handle = projectm::create(&settings, 0);
   
    projectm::set_window_size(projectm_handle, 800, 600);
    projectm::select_random_preset(projectm_handle, true);
    println!("ProjectM -> Initialized");

    // Tests
    // test_get_settings(projectm_handle);
    // test_write_config(projectm_handle);
    // test_get_and_set_texture_size(projectm_handle);
    // test_get_and_set_beat_sensitivity(projectm_handle);
    // test_get_and_set_hard_cut_duration(projectm_handle);
    // test_get_and_set_hard_cut_enabled(projectm_handle);
    // test_get_and_set_hard_cut_sensitivity(projectm_handle);
    // test_get_and_set_soft_cut_duration(projectm_handle);
    // test_get_and_set_preset_duration(projectm_handle);
    // test_get_and_set_mesh_x(projectm_handle);
    // test_get_and_set_mesh_y(projectm_handle);
    // test_get_and_set_mesh_size(projectm_handle);
    // test_get_and_set_fps(projectm_handle);
    // test_get_paths(projectm_handle);
    // test_get_and_set_aspect_correction(projectm_handle);
    // test_get_and_set_easter_egg(projectm_handle);
    // test_get_and_set_window_size(projectm_handle);
    // test_write_debug_image_on_next_frame(projectm_handle);


    // events
    let mut event_pump = sdl_context.event_pump()?;

    // renderLoop
    'running: loop {
        // check for event
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        // generate random audio
        generate_random_audio_data(projectm_handle);

        // projectm::render
        projectm::render_frame(projectm_handle);    
        
        // present/render
        canvas.present();
    }

    // finish okay
    Ok(())
}

fn generate_random_audio_data(projectm_handle: projectm_handle)
{
    let mut pcm_data: [[libc::c_short; 512]; 2] = [[0; 512]; 2];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 512 as libc::c_int {
        if i % 2 as libc::c_int == 1 as libc::c_int {
            pcm_data[0 as libc::c_int as usize][i as usize] =
                -(pcm_data[0 as libc::c_int as usize][i as usize] as
                      libc::c_int) as libc::c_short;
            pcm_data[1 as libc::c_int as usize][i as usize] =
                -(pcm_data[1 as libc::c_int as usize][i as usize] as
                      libc::c_int) as libc::c_short
        }
        i += 1
    };

    projectm::pcm_add_int16(projectm_handle, &pcm_data[0][0], 512)    
}

// -- Tests --
fn test_destroy(projectm_handle: projectm_handle) {
    println!("Test: destroy");
    thread::sleep(time::Duration::from_secs(5));
    projectm::destroy(projectm_handle);
}

fn test_get_settings(projectm_handle: projectm_handle) {
    println!("Test -> get_settings");
    println!("{:?}", projectm::get_settings(projectm_handle));

    // !TODO Figure out how to convert the pointer to useable struct
}

fn test_write_config(projectm_handle: projectm_handle) {
    println!("Test -> write_config");

    let config_file = String::from("./test.config");
    let settings = projectm::get_settings(projectm_handle);

    projectm::write_config(config_file, settings);
}

fn test_get_and_set_texture_size(projectm_handle: projectm_handle) {
    println!("Test -> get_texture_size");
    println!("--texture-size: {}", projectm::get_texture_size(projectm_handle));

    println!("Test -> set_texture_size");
    projectm::set_texture_size(projectm_handle, 256);
    println!("--texture-size: {}", projectm::get_texture_size(projectm_handle));
}

fn test_get_and_set_beat_sensitivity(projectm_handle: projectm_handle) {
    println!("Test -> get_beat_sensitivity");
    println!("--beat-sensitivity: {}", projectm::get_beat_sensitivity(projectm_handle));

    println!("Test -> set_beat_sensitivity");
    projectm::set_beat_sensitivity(projectm_handle, 0.9);
    println!("--beat-sensitivity: {}", projectm::get_beat_sensitivity(projectm_handle));
}

fn test_get_and_set_hard_cut_duration(projectm_handle: projectm_handle) {
    println!("Test -> get_hard_cut_duration");
    println!("--hard_cut_duration: {}", projectm::get_hard_cut_duration(projectm_handle));

    println!("Test -> set_hard_cut_duration");
    projectm::set_hard_cut_duration(projectm_handle, 30.0);
    println!("--hard_cut_duration: {}", projectm::get_hard_cut_duration(projectm_handle));
}

fn test_get_and_set_hard_cut_enabled(projectm_handle: projectm_handle) {
    println!("Test -> get_hard_cut_enabled");
    println!("--hard_cut_enabled: {}", projectm::get_hard_cut_enabled(projectm_handle));

    println!("Test -> set_hard_cut_enabled");
    projectm::set_hard_cut_enabled(projectm_handle, true);
    println!("--hard_cut_enabled: {}", projectm::get_hard_cut_enabled(projectm_handle));
}

fn test_get_and_set_hard_cut_sensitivity(projectm_handle: projectm_handle) {
    println!("Test -> get_hard_cut_sensitivity");
    println!("--hard_cut_sensitivity: {}", projectm::get_hard_cut_sensitivity(projectm_handle));

    println!("Test -> set_hard_cut_sensitivity");
    projectm::set_hard_cut_sensitivity(projectm_handle, 0.2);
    println!("--hard_cut_sensitivity: {}", projectm::get_hard_cut_sensitivity(projectm_handle));
}

fn test_get_and_set_soft_cut_duration(projectm_handle: projectm_handle) {
    println!("Test -> get_soft_cut_duration");
    println!("--soft_cut_duration: {}", projectm::get_soft_cut_duration(projectm_handle));

    println!("Test -> set_soft_cut_duration");
    projectm::set_soft_cut_duration(projectm_handle, 25.0);
    println!("--soft_cut_duration: {}", projectm::get_soft_cut_duration(projectm_handle));
}

fn test_get_and_set_preset_duration(projectm_handle: projectm_handle) {
    // println!("Test -> get_preset_duration");
    // println!("--preset_duration: {}", projectm::get_preset_duration(projectm_handle));

    println!("Test -> set_preset_duration");
    projectm::set_preset_duration(projectm_handle, 45.0);
    // println!("--preset_duration: {}", projectm::get_preset_duration(projectm_handle));
}

fn test_get_and_set_mesh_size(projectm_handle: projectm_handle) {
    println!("Test -> get_mesh_size");
    println!("--mesh_size: {:?}", projectm::get_mesh_size(projectm_handle));

    println!("Test -> set_mesh_size");
    projectm::set_mesh_size(projectm_handle, 128, 80);
    println!("--mesh_size: {:?}", projectm::get_mesh_size(projectm_handle));
}

fn test_get_and_set_fps(projectm_handle: projectm_handle) {
    println!("Test -> get_fps");
    println!("--fps: {}", projectm::get_fps(projectm_handle));

    // println!("Test -> set_fps");
    // projectm::set_fps(projectm_handle, 64);
    // println!("--fps: {}", projectm::get_fps(projectm_handle));
}

fn test_get_paths(projectm_handle: projectm_handle) {
    println!("Test -> get_preset_path");
    println!("--preset_path: {}", projectm::get_preset_path(projectm_handle));

    println!("Test -> get_texture_path");
    println!("--texture_path: {}", projectm::get_texture_path(projectm_handle));

    println!("Test -> get_data_path");
    println!("--data_path: {}", projectm::get_data_path(projectm_handle));
}

fn test_get_and_set_aspect_correction(projectm_handle: projectm_handle) {
    println!("Test -> get_aspect_correction");
    println!("--aspect_correction: {:?}", projectm::get_aspect_correction(projectm_handle));

    println!("Test -> set_aspect_correction");
    projectm::set_aspect_correction(projectm_handle, false);
    println!("--aspect_correction: {:?}", projectm::get_aspect_correction(projectm_handle));
}

fn test_get_and_set_easter_egg(projectm_handle: projectm_handle) {
    println!("Test -> get_easter_egg");
    println!("--easter_egg: {:?}", projectm::get_easter_egg(projectm_handle));

    println!("Test -> set_easter_egg");
    projectm::set_easter_egg(projectm_handle, 0.25);
    println!("--easter_egg: {:?}", projectm::get_easter_egg(projectm_handle));
}

fn test_get_and_set_window_size(projectm_handle: projectm_handle) {
    println!("Test -> get_window_size");
    println!("--window_size: {:?}", projectm::get_window_size(projectm_handle));

    println!("Test -> set_window_size");
    projectm::set_window_size(projectm_handle, 640, 360);
    println!("--window_size: {:?}", projectm::get_window_size(projectm_handle));
}

fn test_write_debug_image_on_next_frame(projectm_handle: projectm_handle) {
    println!("Test -> write_debug_image_on_next_frame");
    projectm::write_debug_image_on_next_frame(projectm_handle);
}