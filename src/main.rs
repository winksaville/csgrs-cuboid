use clap::{Parser,CommandFactory};

type Mesh = csgrs::mesh::Mesh<()>;

#[derive(Parser, Debug)]
#[command(version, about = "Generate a cuboid mesh and write it to an STL file.", long_about = None)]
struct Args {
    /// width of the panel
    #[arg(short, long, default_value = "10.0")]
    width: f64,

    /// length of the panel
    #[arg(short, long, default_value = "100.0")]
    length: f64,

    /// height of the panel
    #[arg(short = 'H', long, default_value = "5.0")]
    height: f64,
}

fn main() {
    let args = Args::parse();
    //println!("args: {:?}", args);

    let clap_metadata = Args::command();
    println!(" arg.version: {}", clap_metadata.get_version().unwrap_or_default());

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
