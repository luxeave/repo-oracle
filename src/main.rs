use std::env;
use std::fs;
use std::path::Path;

// Function to create a textual representation of the directory structure
fn create_directory_structure(dir: &Path, indent: &str) -> String {
    let mut output = String::new();

    // Get the list of files in the directory
    let files: Vec<_> = fs::read_dir(dir).unwrap().collect();

    /*
    The files.iter().enumerate() in Rust is a combination of two methods: iter() and enumerate().
    The iter() method is called on a collection, such as a vector or an array, 
    to create an iterator over the elements of the collection. 
    This iterator allows you to access each element in the collection in sequence 
    without mutating the collection.    
    The enumerate() method is called on an iterator, and it transforms the iterator 
    into another iterator that gives you both the index and the value of the elements 
    of the original iterator. This is useful when you need to know the position of an element 
    in the collection while iterating over it.
    So, files.iter().enumerate() creates an iterator over the files collection 
    that yields a tuple for each element in the collection. 
    The first element of the tuple is the index of the element in the collection, 
    and the second element of the tuple is a reference to the element itself.
     */
    for (index, file) in files.iter().enumerate() {
        /*
        First, it calls the as_ref() method on file. This method is defined for Result types 
        in Rust, and it returns a new Result 
        where the Ok and Err values are references to the values in the original Result. 
        In other words, if file is a Result type, file.as_ref() 
        will return a Result that contains references to the original Ok or Err values.
        The unwrap() method is then called on the result of file.as_ref(). 
        The unwrap() method is also defined for Result types, 
        and it returns the Ok value if the Result is Ok, and panics if the Result is Err. */
        let file = file.as_ref().unwrap();
        
        let file_path = file.path();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        let is_last_file = index == files.len() - 1;
        let is_directory = file_path.is_dir();

        output.push_str(&format!(
            "{}{}{}\n",
            indent,
            if is_last_file { "└── " } else { "├── " },
            file_name
        ));

        if is_directory {
            let sub_indent = if is_last_file {
                format!("{}    ", indent)
            } else {
                format!("{}│   ", indent)
            };
            output.push_str(&create_directory_structure(&file_path, &sub_indent));
        }
    }

    output
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let root_dir_path = if args.len() > 1 {
        args[1].clone()
    } else {
        env::current_dir().unwrap().to_str().unwrap().to_string()
    };

    let root_dir = Path::new(&root_dir_path);

    // Create textual representation of the directory structure
    let directory_structure = create_directory_structure(&root_dir, "");
    println!("Directory Structure:\n{}", directory_structure);

    // Save the directory structure to a file
    let output_file = root_dir.join("directory_structure.txt");
    fs::write(output_file, directory_structure).unwrap();
}