use clap::{CommandFactory, Parser};
use csgrs::traits::CSG;
use nalgebra::Vector3;

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

    /// wall thickness of the panel
    #[arg(short = 'W', long, default_value = "0")]
    wall_thickness: f64,
}

fn main() {
    let args = Args::parse();
    //println!("args: {:?}", args);

    let clap_metadata = Args::command();
    println!(
        " arg.version: {}",
        clap_metadata.get_version().unwrap_or_default()
    );

    // Create a panel/cuboid/rectungular box mesh
    let outside_dimensions = Vector3::new(args.width, args.length, args.height);
    println!("outside_dimenstions: {:?}", outside_dimensions);
    let mut outside: Mesh = Mesh::cuboid(
        outside_dimensions.x,
        outside_dimensions.y,
        outside_dimensions.z,
        None,
    );
    if args.wall_thickness > 0.0 {
        // Create a hollow cuboid mesh by subtracting the inside from the outside
        let inside_dimensions = outside_dimensions
            - Vector3::new(
                args.wall_thickness * 2.0,
                args.wall_thickness * 2.0,
                args.wall_thickness * 2.0,
            );
        println!("inside_dimenstions: {:?}", inside_dimensions);
        let inside: Mesh = Mesh::cuboid(
            inside_dimensions.x,
            inside_dimensions.y,
            inside_dimensions.z,
            None,
        )
        .translate(
            args.wall_thickness,
            args.wall_thickness,
            args.wall_thickness,
        );

        outside = outside.difference(&inside);
        let center_dimensions = (outside_dimensions + inside_dimensions) / 2.0;
        println!("center_dimensions: {:?}", center_dimensions);
    }

    // Write the result as an ASCII STL:
    let name = "cuboid".to_owned()
        + &format!("_w-{:.2}", args.width)
        + &format!("_l-{:.2}", args.length)
        + &format!("_h-{:.2}", args.height);
    let stl = outside.to_stl_ascii(&name);
    std::fs::write(name.to_owned() + ".stl", stl).unwrap();
}
