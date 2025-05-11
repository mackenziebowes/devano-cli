use crate::library::colors::transforms;
use std::fs;
use std::path::Path;
use anyhow::Result;

pub fn write_devano_palette_rust(palette: &transforms::DevanoPalette) -> Result<()> {
    // this function is for use in the Devano CLI source code - it outputs a full palette definition to a rust file
    // which the dev can that integrate (and name) into the named default palettes
    let dir_path = Path::new("src").join("codegen").join("palettes");
    fs::create_dir_all(&dir_path)?;
    let file_name = format!("newest.rs");
    let file_path = dir_path.join(&file_name);
    let rust_code = format!(
        "
        pub const NEW_PALETTE: DevanoPalette = DevanoPalette {{
    kora: NeutralHexes {{
        ara: \"{}\".to_string(),
        ene: \"{}\".to_string(),
        izi: \"{}\".to_string(),
        ona: \"{}\".to_string(),
    }},
    aleva: NeutralHexes {{
        ara: \"{}\".to_string(),
        ene: \"{}\".to_string(),
        izi: \"{}\".to_string(),
        ona: \"{}\".to_string(),
    }},
    ara: AccentHexes {{
        ara: \"{}\".to_string(),
        ene: \"{}\".to_string(),
        izi: \"{}\".to_string(),
    }},
    ene: AccentHexes {{
        ara: \"{}\".to_string(),
        ene: \"{}\".to_string(),
        izi: \"{}\".to_string(),
    }},
    izi: AccentHexes {{
        ara: \"{}\".to_string(),
        ene: \"{}\".to_string(),
        izi: \"{}\".to_string(),
    }},
    ona: AccentHexes {{
        ara: \"{}\".to_string(),
        ene: \"{}\".to_string(),
        izi: \"{}\".to_string(),
    }},
    uvo: AccentHexes {{
        ara: \"{}\".to_string(),
        ene: \"{}\".to_string(),
        izi: \"{}\".to_string(),
    }},
    bala: AccentHexes {{
        ara: \"{}\".to_string(),
        ene: \"{}\".to_string(),
        izi: \"{}\".to_string(),
    }},
}};",
        palette.kora.ara, palette.kora.ene, palette.kora.izi, palette.kora.ona,
        palette.aleva.ara, palette.aleva.ene, palette.aleva.izi, palette.aleva.ona,
        palette.ara.ara, palette.ara.ene, palette.ara.izi,
        palette.ene.ara, palette.ene.ene, palette.ene.izi,
        palette.izi.ara, palette.izi.ene, palette.izi.izi,
        palette.ona.ara, palette.ona.ene, palette.ona.izi,
        palette.uvo.ara, palette.uvo.ene, palette.uvo.izi,
        palette.bala.ara, palette.bala.ene, palette.bala.izi,
    );
    fs::write(&file_path, rust_code)?;

    println!("Palette written to {:?}", file_path);
    Ok(())
}

pub fn write_devano_palette_css(palette: &transforms::DevanoPalette) -> Result<()> {
    // prep the file
    let dir_path = Path::new("src").join("css").join("devano");
    fs::create_dir_all(&dir_path)?;
    let file_name = format!("palette.css");
    let file_path = dir_path.join(&file_name);

     // Helper function to generate CSS variables for a given set of colors
     fn generate_neutrals(prefix: &str, colors: &transforms::NeutralHexes) -> Vec<String> {
        vec![
            format!("--{}-a: {};", prefix, colors.ara),
            format!("--{}-e: {};", prefix, colors.ene),
            format!("--{}-i: {};", prefix, colors.izi),
            format!("--{}-o: {};", prefix, colors.ona)
        ]
    }

    fn generate_accents(prefix: &str, colors: &transforms::AccentHexes) -> Vec<String> {
        vec![
            format!("--{}-a: {};", prefix, colors.ara),
            format!("--{}-e: {};", prefix, colors.ene),
            format!("--{}-i: {};", prefix, colors.izi)
        ]
    }

    let mut css_lines = Vec::new();

    css_lines.push(":root {\n".to_string());
    css_lines.extend(
        generate_neutrals("bg", &palette.aleva)
            .into_iter()
            .map(|line| format!("\t{}\n", line))
    );
    css_lines.extend(
        generate_neutrals("fg", &palette.kora)
            .into_iter()
            .map(|line| format!("\t{}\n", line))
    );
    css_lines.extend(
        generate_accents("c-a", &palette.ona)
            .into_iter()
            .map(|line| format!("\t{}\n", line))
    );
    css_lines.extend(
        generate_accents("c-e", &palette.uvo)
            .into_iter()
            .map(|line| format!("\t{}\n", line))
    );
    css_lines.extend(
        generate_accents("c-i", &palette.bala)
            .into_iter()
            .map(|line| format!("\t{}\n", line))
    );
    css_lines.push("}\n".to_string());

    css_lines.push("@media (prefers-color-scheme: dark) {\n\t:root {\n".to_string());
    css_lines.extend(
        generate_neutrals("bg", &palette.kora)
            .into_iter()
            .map(|line| format!("\t\t{}\n", line))
    );
    css_lines.extend(
        generate_neutrals("fg", &palette.aleva)
            .into_iter()
            .map(|line| format!("\t\t{}\n", line))
    );
    css_lines.extend(
        generate_accents("c-a", &palette.ara)
            .into_iter()
            .map(|line| format!("\t\t{}\n", line))
    );
    css_lines.extend(
        generate_accents("c-e", &palette.ene)
            .into_iter()
            .map(|line| format!("\t\t{}\n", line))
    );
    css_lines.extend(
        generate_accents("c-i", &palette.izi)
            .into_iter()
            .map(|line| format!("\t\t{}\n", line))
    );
    css_lines.push("\t}\n}".to_string());

    // Join all lines into a single CSS string
    let css = css_lines.join("");

    // Write the CSS to the file
    fs::write(&file_path, css)?;

    println!("Palette written to {:?}", file_path);
    Ok(())
}