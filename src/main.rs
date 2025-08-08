use clap::{ArgAction, Parser};

type Mesh = csgrs::mesh::Mesh<()>;

#[derive(Parser, Debug)]
#[clap(disable_help_flag = true)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Overried the default help message
    #[clap(short='H', long, action = ArgAction::Help, help = "Prints help information")]
    help: Option<bool>,

    /// width of the panel
    #[clap(short, long, default_value = "10.0")]
    width: f64,

    /// length of the panel
    #[clap(short, long, default_value = "100.0")]
    length: f64,

    /// height of the panel
    #[clap(short = 'h', long, default_value = "5.0")]
    height: f64,
}

fn main() {
    let args = Args::parse();

    // Create a panel/cuboid/rectungular box mesh
    let panel: Mesh = Mesh::cuboid(args.width, args.length, args.height, None);

    // Write the result as an ASCII STL:
    let name = "cuboid".to_owned()
        + &format!("_w-{:.2}", args.width)
        + &format!("_l-{:.2}", args.length)
        + &format!("_h-{:.2}", args.height);
    let stl = panel.to_stl_ascii(&name);
    std::fs::write(name.to_owned() + ".stl", stl).unwrap();
}
