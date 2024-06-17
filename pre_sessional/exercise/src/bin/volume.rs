fn main() {
    let pi= 3.142;

    let sphere_radius = 1.0;
    let cylinder_radius = 1.0;
    let cylinder_height = 1.0;
    let cone_radius = 1.0;
    let cone_height = 1.0;

    let sphere_volume = (4.0/3.0)*pi*sphere_radius*sphere_radius*sphere_radius;
    let cylinder_volume = pi*cylinder_radius*cylinder_radius*cylinder_height;
    let cone_volume = pi*cone_radius*cone_radius*(cone_height/3.0);

    println!("The volume of sphere is {sphere_volume}
    The volume of cylinder is {cylinder_volume}
    The volume of cone is {cone_volume}.")
}