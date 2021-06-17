use std::{borrow::{Borrow, BorrowMut}, cell::{RefCell, RefMut}, cmp::{max, min}, fmt::Debug, sync::{Arc, Mutex, RwLock, atomic::{AtomicBool, Ordering}}, time::{Duration, Instant}};

use captrs::{Bgr8, Capturer};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EffectDir {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}


#[repr(C, packed)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Colour([u8; 3]);

impl Colour {
    pub const fn new() -> Self {
        Self([0,0,0])
    }

    pub const fn new_colour(r: u8, g: u8, b: u8) -> Self {
        Self([r,g,b])
    }

    // Creates an interoplated gradient between 2 colours, with [steps] entries
    // Returning entires size will always be steps + 2
    pub fn gradient(&self, other: &Colour, steps: usize) -> Vec<Colour> {
        /*
        let mut ret = vec![Colour::default(); steps+2];
        ret[0] = self.clone();
        ret[steps+1] = other.clone();
        let d_r = ((other.r - self.r) as f32) / steps as f32;
        let d_g = ((other.g - self.g) as f32) / steps as f32;
        let d_b = ((other.b - self.b) as f32) / steps as f32;

        let mut tmp: &Colour = ret[0].borrow();

        for x in 1..steps+2 {
            ret[x] = Colour (
                [((tmp.0[0] as f32) + d_r) as u8,
                ((tmp.0[1] as f32) + d_g) as u8,
                ((tmp.0[2] as f32) + d_b) as u8]
            );
            tmp = ret[x].borrow();
        }
        */



        Vec::new()
    }
}

pub struct EffectLayer<const X: usize, const Y: usize> {
    internal_matrix: [[Colour; X]; Y], // This gets sent for updating!

    /// Key Matrix
    pub matrix: [[Colour;  X]; Y],
    /// Mask Matrix. Any keys with false, the effect is NOT applied to
    mask: [[bool; X]; Y]
}

impl<const X: usize, const Y: usize> EffectLayer<X, Y> {
    pub const fn create_blank(mask: [[bool; X]; Y]) -> Self {
        Self {
            mask,
            internal_matrix: [[Colour::new(); X]; Y],
            matrix: [[Colour::new(); X]; Y]
        }
    }

    /// Returns None if mask is not applied to the key in this location
    pub fn get_key(&self, x: usize, y: usize) ->Option<&Colour> {
        match self.mask[x][y] {
            false => None,
            true => Some(&self.matrix[x][y])
        }
    }

    pub fn update_matrix(&mut self) -> &[[Colour; X]; Y] {
        for y in 0..Y {
            for x in 0..X {
                if self.mask[x][y] {
                    self.internal_matrix[x][y] = self.matrix[x][y]
                }
            }
        }
        &self.internal_matrix
    }

    pub fn get_width(&self) -> u32 {
        X as u32
    }

    pub fn get_height(&self) -> u32 {
        Y as u32
    }

    pub fn shift_matrix_vertical(&mut self, up: bool) {
        let old = self.matrix.clone();
        if up {
            let tmp = old[0];
            self.matrix[0..Y-1].copy_from_slice(&old[1..]);
            self.matrix[Y-1] = tmp;
        } else {
            let tmp = self.matrix[Y-1];
            self.matrix[1..].copy_from_slice(&old[0..Y-1]);
            self.matrix[0] = tmp;
        }
    }

    pub fn shift_matrix_horizontal(&mut self, left: bool) {
        for row in self.matrix.iter_mut() {
            let old = row.clone();
            if left {
                let tmp = row[0];
                row[0..X-1].copy_from_slice(&old[1..]);
                row[X-1] = tmp;
            } else {
                let tmp = row[X-1];
                row[1..].copy_from_slice(&old[0..X-1]);
                row[0] = tmp;
            }
        }
    }

    pub fn set_matrix_bg(&mut self, c: Colour) {
        self.matrix = [[c; X]; Y];
    }

    pub fn clear_matrix(&mut self) {
        self.matrix = [[Colour::new_colour(0, 0, 0); X]; Y];
    }
}

pub trait Effect<const X: usize, const Y: usize> where Self: Debug + Sized + Clone {
    fn init(&mut self, layer: &mut EffectLayer<X, Y>) where Self: Sized;
    fn update(&mut self, matrix: &mut EffectLayer<X, Y>) where Self: Sized;
}

#[derive(Debug, Clone, Copy)]
pub struct StaticEffect {
    colour: Colour
}

impl<const X: usize, const Y: usize> Effect<X, Y> for StaticEffect {
    // Called once when effect starts, update will be called 50ms after
    fn init(&mut self, layer: &mut EffectLayer<X, Y>) {
        
    }

    // Called 20 times per second
    fn update(&mut self, matrix: &mut EffectLayer<X, Y>) {
        todo!()
    }
}

const WAVE_EFFECT_MAX_SPD: u32 = 10;

#[derive(Clone)]
pub struct CaptureDisplayEffect {
    run: bool,
    thread_run: Arc<AtomicBool>,
    img: Arc<RwLock<DynamicImage>>,
    capture_width: u32,
    capture_height: u32
}

impl Debug for CaptureDisplayEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CaptureDisplayEffect").finish()
    }
}

impl CaptureDisplayEffect {
    pub fn new(x: u32, y: u32) -> Option<Self> {

        let should_run = Arc::new(AtomicBool::new(true));
        let should_run_t = should_run.clone();

        let cap2 = Capturer::new(0).ok()?;
        let (width, height) = cap2.geometry();
        let (width_t, height_t) = (width, height);

        let dyn_img : Arc<RwLock<DynamicImage>> = Arc::new(RwLock::new(DynamicImage::new_rgb8(width, height)));
        let dym_img_t: Arc<RwLock<DynamicImage>> = dyn_img.clone();

        std::thread::spawn(move||{
            if let Ok(mut cap) = Capturer::new(0) {
                let interval = Duration::from_millis(40);
                while should_run_t.load(Ordering::Relaxed) {
                    let mut buf = Vec::with_capacity((width_t * height_t) as usize * 3);
                    let start = Instant::now();
                    if cap.capture_store_frame().is_ok() {
                        for px in cap.get_stored_frame().unwrap() {
                            buf.extend_from_slice(&[px.r, px.g, px.b]);
                        }
                        let img = DynamicImage::ImageRgb8(ImageBuffer::<Rgb<u8>, Vec<u8>>::from_vec(width_t, height_t, buf).unwrap());
                        *dym_img_t.write().unwrap() = img.resize_to_fill(x, y, image::imageops::FilterType::Triangle);
                    }
                    if let Some(remain) = interval.checked_sub(start.elapsed()) {
                        std::thread::sleep(remain);
                    }
                }
            }
        });

        Some(Self { 
            run: true, 
            thread_run: Arc::new(AtomicBool::new(true)), 
            img: dyn_img,
            capture_width: width,
            capture_height: height
        })
    }
}

impl<const X: usize, const Y: usize> Effect<X, Y> for CaptureDisplayEffect {
    // Called once when effect starts, update will be called 50ms after
    fn init(&mut self, layer: &mut EffectLayer<X, Y>) {
        layer.clear_matrix();
    }

    // Called 20 times per second
    fn update(&mut self, matrix: &mut EffectLayer<X, Y>) {
        if !self.run {
            self.run = true;
            return;
        }

        if let Ok(frame) = self.img.read() {
            println!("{} {}", frame.width(), frame.height());
            for (y, row) in matrix.matrix.iter_mut().enumerate() {
                for (x, key) in row.iter_mut().enumerate() {

                    let sample_pixel = frame.get_pixel(x as u32, y as u32);
                    key.0.copy_from_slice(&sample_pixel.0[0..3]);
                }
            }
        }
        self.run = false;
    }
}




#[derive(Debug, Clone)]
pub struct WaveEffect {
    dir: EffectDir,
    speed: u32,
    spectrum: Vec<Colour>
}

impl WaveEffect {
    /// Speed - 1-10, lower = slower wave
    pub fn new(dir: EffectDir, speed: u32) -> Self {
        let mut spd = speed;
        if spd == 0 {
            spd = 1;
        } else if spd > WAVE_EFFECT_MAX_SPD {
            spd = WAVE_EFFECT_MAX_SPD;
        }
        Self {
            dir,
            speed: spd,
            spectrum: Vec::new()
        }
    }    
}

impl<const X: usize, const Y: usize> Effect<X, Y> for WaveEffect {
    // Called once when effect starts, update will be called 50ms after
    fn init(&mut self, layer: &mut EffectLayer<X, Y>) {
        
    }

    // Called 20 times per second
    fn update(&mut self, matrix: &mut EffectLayer<X, Y>) {
        todo!()
    }
}