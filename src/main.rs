use arboard::Clipboard;
use clap::Parser;
use std::io::{self, Read};
use typst::foundations::Bytes;
use typst::text::Font;
use typst_as_lib::TypstTemplate;
use typst_render::render;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(help = "Typst code to compile")]
    typst_code: Option<String>,
}

static FONT: &[u8] = include_bytes!("../fonts/NewCMMath-Regular.otf");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let font = Font::new(Bytes::from(FONT), 0).expect("Could not parse font!");
    let args = Args::parse();

    // Collect Typst code, either from CLI arg or stdin
    let typst_code = match args.typst_code {
        Some(code) => code,
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
    };

    let code = format!(
        "
    #import \"@preview/physica:0.9.3\": *
    #set page(margin: 10pt, height: auto, width: auto)

    $
    {typst_code}
    $"
    );

    // Read in fonts and the main source file.
    // We can use this template more than once, if needed (Possibly
    // with different input each time).
    let template = TypstTemplate::new(vec![font], code).with_package_file_resolver(None);

    // Run it
    let doc = template
        .compile()
        .output
        .expect("typst::compile() returned an error!");

    // Render first page
    let page = doc.pages.first().ok_or("No pages in document")?;
    let png = render(page, 3.0);

    // Copy to clipboard
    let mut clipboard = Clipboard::new()?;
    clipboard.set_image(arboard::ImageData {
        width: png.width() as usize,
        height: png.height() as usize,
        bytes: png.data().into(),
    })?;

    println!("Image copied to clipboard successfully!");
    Ok(())
}
