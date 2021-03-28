pub fn read() {
    //here is some nice copypasta straight from the docs

    let filename = "etc/layout.conf";

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        //throw error if file not found/not readable
        .expect("[artemis file i/o] Error while reading file");

    println!("With text:\n{}", contents);
}
