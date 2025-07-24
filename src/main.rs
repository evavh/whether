mod files;

fn main() {
    let file =
        hdf5_metno::File::open("input/RAD_NL25_RAC_FM_202507201425.h5").unwrap();
    let groups = file.groups().unwrap();
    dbg!(groups);
}
