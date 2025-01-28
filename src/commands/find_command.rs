pub fn execute(pinpoint: String, key: String, auto: bool) {
    if auto {
        println!("Automatically traveling to the directory if only one result for key: {}", key);
    } else {
        println!("Finding: {}, with key: {}", pinpoint, key);
    }
}
