pub mod zerocss {
    use zerocss_finder::search;
    use zerocss_extractor::extract;
    
    use std::fs::read_to_string;

    pub fn compile() {
        let files = search("./**/*.rs");
    
        println!("{:?}", files);
    
        for file in files {
            let output = &read_to_string(file).expect("Failed to read file");
            let classes = extract(output);
    
            // println!("{:?}", output);
            println!("{:?}", classes);
    
            // println!("{}", file.to_string_lossy());
        }
    }
}
