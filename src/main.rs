use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    version = "v0.1.0",
    about = "create a mediawiki table for a map from a CSV provided on stdin"
)]
struct Args {
    /// If provided, replace this character in the map with an inline sprite with the given filename
    #[clap(long, default_value_t = 's')]
    sprite_char: char,
    /// The filename of the sprite
    #[clap(long, requires = "sprite_alt")]
    sprite: Option<String>,
    /// Size of the sprite in px
    #[clap(long, requires = "sprite", default_value_t = 15)]
    sprite_px: usize,
    /// Alt text for the sprite
    #[clap(long, requires = "sprite")]
    sprite_alt: Option<String>,
    /// Attributes e.g. 'class="mytable"' to apply to the whole table
    #[clap(long)]
    table_attrs: Option<String>,
    /// Size of a (square) table cell in px
    #[clap(long, default_value_t = 40)]
    cell_size_px: usize,
    /// Character to denote an area outside of the map
    #[clap(long, default_value_t = 'x')]
    oob_char: char,
    /// Color hex code to fill the out-of-bound cells
    #[clap(long, default_value = "e6caa2")]
    oob_color_hex: String,
    /// Character to denote water on the map
    #[clap(long, default_value_t = 'w')]
    water_char: char,
    /// Color hex code to fill the water cells
    #[clap(long, default_value = "b2ccd3")]
    water_color_hex: String,
}

fn sprite_tag(sprite_fn: &str, sprite_px: usize, sprite_alt: &str) -> String {
    format!("[[File:{}|{}px|{}]]", sprite_fn, sprite_px, sprite_alt)
}

fn color(color: &str) -> String {
    format!(r#"style="background:#{};" |"#, color)
}

fn header_cell(cell_size_px: usize, contents: &str) -> String {
    format!(r#"! style="width: {}px;" |{}"#, cell_size_px, contents)
}

fn first_cell(cell_size_px: usize, contents: &str) -> String {
    format!(
        r#"! style="width: {}px; height: {}px" | {}"#,
        cell_size_px, cell_size_px, contents
    )
}

fn begin_row_cell(cell_size_px: usize, contents: &str) -> [String; 2] {
    [
        "|-".into(),
        format!(r#"! style="height: {}px;" |{}"#, cell_size_px, contents),
    ]
}

fn cell(contents: &str) -> String {
    format!("| {}", contents)
}

fn begin_table(attrs: &str) -> String {
    format!("{{| {}", attrs)
}

fn end_table() -> String {
    "|}".into()
}

fn build_cell_contents(raw: &str, args: &Args) -> String {
    let mut building = String::new();
    for chr in raw.chars() {
        if chr == args.oob_char {
            building.push_str(&color(&args.oob_color_hex));
        } else if chr == args.water_char {
            building.push_str(&color(&args.water_color_hex));
        } else if chr == args.sprite_char {
            if let Some(ref spr) = args.sprite {
                building.push_str(&sprite_tag(
                    spr,
                    args.sprite_px,
                    args.sprite_alt.as_deref().unwrap(),
                ));
            }
        } else {
            building.push(chr);
        }
    }
    building
}

fn main() {
    let args = Args::parse();
    let mut table_rows: Vec<String> = vec![];
    table_rows.push(begin_table(args.table_attrs.as_deref().unwrap_or_default()));
    let mut rdr = csv::Reader::from_reader(std::io::stdin());
    let header = rdr.headers().unwrap();
    let mut header_iter = header.into_iter();
    let first_header = header_iter.next().unwrap();
    table_rows.push(first_cell(args.cell_size_px, first_header));
    for h in header_iter {
        table_rows.push(header_cell(args.cell_size_px, h));
    }

    for result in rdr.records() {
        let record = result.unwrap();
        let mut record_iter = record.into_iter();
        let first = record_iter.next().unwrap();
        table_rows.extend_from_slice(&begin_row_cell(
            args.cell_size_px,
            &build_cell_contents(first, &args),
        ));
        for raw_cell in record_iter {
            table_rows.push(cell(&build_cell_contents(raw_cell, &args)));
        }
    }

    table_rows.push(end_table());
    println!("{}", table_rows.join("\n"));
}
