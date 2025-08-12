use clap::{CommandFactory, Parser};
use csgrs::{float_types::Real, traits::CSG};
use nalgebra::Vector3;

type Mesh = csgrs::mesh::Mesh<()>;

// TODO: Forbid dimenstions negative dimensions
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
    #[arg(short = 't', long, default_value = "0")]
    wall_thickness: f64,

    /// Gyroid infill resolution
    #[arg(short = 'r', long, default_value = "0")]
    resolution: usize,

    /// Gyroid infill period
    /// This is the distance between the repeating patterns in the gyroid infill.
    #[arg(short = 'p', long, default_value = "2.0")]
    period: f64,

    /// Gyroid infill isovalue
    /// This is the value that determines the density of the gyroid infill.
    #[arg(short = 'i', long, default_value = "0.0")]
    iso_value: f64,
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

    // Create a hollow cuboid mesh by subtracting the inside from the outside
    let inside_dimensions = outside_dimensions
        - Vector3::new(
            args.wall_thickness * 2.0,
            args.wall_thickness * 2.0,
            args.wall_thickness * 2.0,
        );
    println!("inside_dimenstions: {:?}", inside_dimensions);

    if args.wall_thickness > 0.0 {
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
    }

    if args.resolution > 0 {
        println!(
            "Generating gyroid infill with resolution: {}, period: {:.2}, iso_value: {:.2}",
            args.resolution, args.period, args.iso_value
        );

        let center_dimensions = (outside_dimensions + inside_dimensions) / 2.0;
        //let center_dimensions = inside_dimensions;
        println!("center_dimensions: {:?}", center_dimensions);

        // Generate an infill pattern with the ceter_dimensions size
        let mut infill_cuboid = Mesh::cuboid(
            center_dimensions.x,
            center_dimensions.y,
            center_dimensions.z,
            None,
        )
        .translate(
            args.wall_thickness,
            args.wall_thickness,
            args.wall_thickness,
        );
        let resolution: usize = args.resolution; // resolution of the gyroid infill
        let period: Real = args.period as Real; // period of the gyroid infill
        let iso_value: Real = args.iso_value as Real; // isovalue of the gyroid infill
        infill_cuboid = infill_cuboid.gyroid(resolution, period, iso_value, None);
    
        outside = outside.union(&infill_cuboid);
    }

    let infill_names = if args.resolution > 0 {
            format!("_r-{:.2}", args.resolution) 
            + &format!("_p-{:.2}", args.period)
            + &format!("_i-{:.2}", args.iso_value)
        } else {
            String::new()
        };

    // Write the result as an ASCII STL:
    let name = "cuboid".to_owned()
        + &format!("_w-{:.2}", args.width)
        + &format!("_l-{:.2}", args.length)
        + &format!("_h-{:.2}", args.height)
        + &format!("_t-{:.2}", args.wall_thickness)
        + &infill_names; 
    let stl = outside.to_stl_ascii(&name);
    std::fs::write(name.to_owned() + ".stl", stl).unwrap();
}
