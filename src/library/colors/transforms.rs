use palette::{FromColor, IntoColor, LinSrgb, Srgb, Mix, ShiftHue, blend::Blend, Oklch};
use std::fmt;

#[derive(Debug)]
pub struct PaletteError;

impl fmt::Display for PaletteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Something went wrong")
    }
}

impl std::error::Error for PaletteError {}

#[derive(Debug)]
pub struct DevanoPalette {
    pub kora: NeutralHexes,
    pub aleva: NeutralHexes,
    pub ara: AccentHexes,
    pub ene: AccentHexes,
    pub izi: AccentHexes,
    pub ona: AccentHexes,
    pub uvo: AccentHexes,
    pub bala: AccentHexes,
}

#[derive(Debug)]
pub struct NeutralHexes {
    pub ara: String,
    pub ene: String,
    pub izi: String,
    pub ona: String,
}

#[derive(Debug)]
pub struct AccentHexes {
    pub ara: String,
    pub ene: String,
    pub izi: String,
}

pub fn make_simple_devano_palette(hex_color: &str) -> Result<DevanoPalette, PaletteError> {
    let rgb = parse_hex(hex_color);
    let main_oklch: Oklch = rgb.into_color();

    let desaturated_oklch: Oklch = Oklch::new(0.5, 0.0, main_oklch.hue);
    let darkest_oklch: Oklch = Oklch::new(0.0, 0.00, main_oklch.hue);
    let brightest_oklch: Oklch = Oklch::new(1.0, 0.00, main_oklch.hue);

    let dark_mix: [Oklch; 32] = take_n_mix(darkest_oklch, brightest_oklch, 32)
        .try_into()
        .expect("Expected exactly 10 values");
    let light_mix: [Oklch; 32] = take_n_mix(desaturated_oklch, brightest_oklch, 32).try_into().expect("Expected exactly 10 values");
    // make a blend
    let dark_tones: [Oklch; 4] = [dark_mix[3], dark_mix[6], dark_mix[9], dark_mix[12]];
    let light_tones: [Oklch; 4] = [light_mix[23], light_mix[25], light_mix[27], light_mix[30]];
    let accents = make_accents_simple(&main_oklch);
    let dark_tokens: [String; 4] = dark_tones.map(|tone| to_hex(&tone));
    let light_tokens: [String; 4] = light_tones.map(|tone| to_hex(&tone));
    let palette = DevanoPalette {
        kora: NeutralHexes {
            ara: dark_tokens[0].clone(),
            ene: dark_tokens[1].clone(),
            izi: dark_tokens[2].clone(),
            ona: dark_tokens[3].clone(),
        },
        aleva: NeutralHexes {
            ara: light_tokens[0].clone(),
            ene: light_tokens[1].clone(),
            izi: light_tokens[2].clone(),
            ona: light_tokens[3].clone(),
        },
        ara: AccentHexes {
            ara: to_hex(&accents.hue1_dark.dark),
            ene: to_hex(&accents.hue1_dark.mid),
            izi: to_hex(&accents.hue1_dark.light),
        },
        ene: AccentHexes {
            ara: to_hex(&accents.hue2_dark.dark),
            ene: to_hex(&accents.hue2_dark.mid),
            izi: to_hex(&accents.hue2_dark.light),
        },
        izi: AccentHexes {
            ara: to_hex(&accents.hue3_dark.dark),
            ene: to_hex(&accents.hue3_dark.mid),
            izi: to_hex(&accents.hue3_dark.light),
        },
        ona: AccentHexes {
            ara: to_hex(&accents.hue1_light.dark),
            ene: to_hex(&accents.hue1_light.mid),
            izi: to_hex(&accents.hue1_light.light),
        },
        uvo: AccentHexes {
            ara: to_hex(&accents.hue2_light.dark),
            ene: to_hex(&accents.hue2_light.mid),
            izi: to_hex(&accents.hue2_light.light),
        },
        bala: AccentHexes {
            ara: to_hex(&accents.hue3_light.dark),
            ene: to_hex(&accents.hue3_light.mid),
            izi: to_hex(&accents.hue3_light.light),
        },
    };
    Ok(palette)
}

pub fn take_n_mix(color_a: Oklch, color_b: Oklch, n: usize) -> Vec<Oklch> {
    let mut steps = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f32 / (n - 1) as f32; // Normalize to range [0, 1]
        steps.push(color_a.mix(color_b, t));
    };
    steps
}

// 36334D => .21, .19, .25
pub fn parse_hex(hex_code: &str) -> Srgb<f32> {
    let hex = hex_code.strip_prefix("#").unwrap_or(hex_code);
    let expanded_hex = match hex.len() {
        1 => hex.repeat(6),
        2 => hex.repeat(3),
        3 => hex.chars().flat_map(|c| std::iter::repeat(c).take(2)).collect::<String>(),
        6 => hex.to_string(),
        _ => panic!("Invalid hex code format!"),
    };

    let (r, g, b) = (
        &expanded_hex[0..2],
        &expanded_hex[2..4],
        &expanded_hex[4..6],
    );
    // u8 => unsigned 8 bit integer => 0, 255 (256)
    let red = u8::from_str_radix(r, 16).expect("Invalid red component") as f32 / 255.0;
    let green = u8::from_str_radix(g, 16).expect("Invalid green component") as f32 / 255.0;
    let blue = u8::from_str_radix(b, 16).expect("Invalid blue component") as f32 / 255.0;

    // Step 8: Construct an Srgb from the red, green, and blue components
    Srgb::new(red, green, blue)
}


pub fn to_hex(color: &Oklch) -> String {
    // 1. Convert to Srgb
    let srgb: Srgb<u8> = Srgb::from_color(*color).into_format();

    // 2. Explode to components
    let (r, g, b) = (srgb.red, srgb.green, srgb.blue);

    // 3. Convert components to hex
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

pub struct Accent {
    pub hue1_dark: AccentShades,
    pub hue2_dark: AccentShades,
    pub hue3_dark: AccentShades,
    pub hue1_light: AccentShades,
    pub hue2_light: AccentShades,
    pub hue3_light: AccentShades,
}

pub struct AccentShades {
    pub dark: Oklch,
    pub mid: Oklch,
    pub light: Oklch,
}

pub fn make_accents_simple(color: &Oklch) -> Accent {
    let hue_forward = color.shift_hue(90.0);
    let hue_steps: [Oklch; 10] = take_n_mix(*color, hue_forward, 10)
    .try_into()
    .expect("Expected exactly 10 values");
    let hue2 = hue_steps[3];
    let hue3 = hue_steps[6];
    let all_hues: [Oklch; 3] = [*color, hue2, hue3];

    let create_shades = |base: Oklch| {
        let base_rgb: Srgb = base.into_color(); // can't cast Oklch to LinSrgb
        let base_lrgb: LinSrgb = base_rgb.into_color();
        let dark_lrgb = base_lrgb.multiply(base_lrgb); // Multiply
        let light_lrgb = base_lrgb.screen(base_lrgb); // Screen
        let dark_rgb: Srgb = dark_lrgb.into_color(); // cast BACK to rgb
        let light_rgb: Srgb = light_lrgb.into_color();
        AccentShades {
            dark: dark_rgb.into_color(), // finally, cast back to HSL
            mid: base,
            light: light_rgb.into_color(),
        }
    };

    let hue1_dark_shades = create_shades(Oklch::new(0.5, all_hues[0].chroma, all_hues[0].hue));
    let hue2_dark_shades = create_shades(Oklch::new(0.5, all_hues[1].chroma, all_hues[1].hue));
    let hue3_dark_shades = create_shades(Oklch::new(0.5, all_hues[2].chroma, all_hues[2].hue));
    let hue1_light_shades = create_shades(Oklch::new(0.9, all_hues[0].chroma, all_hues[0].hue));
    let hue2_light_shades = create_shades(Oklch::new(0.9, all_hues[1].chroma, all_hues[1].hue));
    let hue3_light_shades = create_shades(Oklch::new(0.9, all_hues[2].chroma, all_hues[2].hue));
    // let hue1_dark_shades = create_shades(all_hues[0].darken_fixed(1.0).lighten_fixed(0.2));
    // let hue2_dark_shades = create_shades(all_hues[1].darken_fixed(1.0).lighten_fixed(0.2));
    // let hue3_dark_shades = create_shades(all_hues[2].darken_fixed(1.0).lighten_fixed(0.2));
    // let hue1_light_shades = create_shades(all_hues[0].lighten_fixed(1.0).darken_fixed(0.2));
    // let hue2_light_shades = create_shades(all_hues[1].lighten_fixed(1.0).darken_fixed(0.2));
    // let hue3_light_shades = create_shades(all_hues[2].lighten_fixed(1.0).darken_fixed(0.2));
    // reversed for use with appropriate dark/light mode
    Accent {
        hue1_light: hue1_dark_shades,
        hue2_light: hue2_dark_shades,
        hue3_light: hue3_dark_shades,
        hue1_dark: hue1_light_shades,
        hue2_dark: hue2_light_shades,
        hue3_dark: hue3_light_shades,
    }
}