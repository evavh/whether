use std::ops::Deref;

use hdf5_metno::Group;

mod files;

fn main() {
    let file = hdf5_metno::File::open("input/RAD_NL25_RAC_FM_202507201425.h5")
        .unwrap();
    dbg!(hdf5_metno::HDF5_VERSION);
    let image1 = file.group("image1").unwrap();
    let image1 = image1.dataset("image_data").unwrap();
    let image1 = image1.read_2d::<u16>().unwrap();
    dbg!(image1);
}

fn print_subgroups(group: Group) {
    if group.is_empty() {
        println!("{}", group.name());
        return;
    }

    let groups = group.groups().unwrap();
    for group in groups {
        print_subgroups(group);
    }
}
