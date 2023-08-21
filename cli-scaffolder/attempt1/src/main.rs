use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    println!("Project builder app!");

    // let input = ".";

    make_proj1()?;

    Ok(())
}

const TESTING_DIR: &str = "testing_dir";
const TEMPLATES_DIR: &str = "templates";

fn make_proj1() -> io::Result<()> {
    let testing_dir = Path::new(TESTING_DIR);

    // Clean all insides
    if testing_dir.exists() {
        remove_contents(testing_dir)?;
    }
    // Now testing_dir should be empty

    println!("The testing_dir {:?} should now be empty", testing_dir);



    // Copy the template chosen template
    let which = "idk";
    copy_template_into(which, testing_dir)?;



    Ok(())
}


fn copy_template_into(template: &str, into: &Path) -> io::Result<()> {
    println!("Entered copy template");
    let chosen_template = Path::new(TEMPLATES_DIR).join(template);
    println!("Constructed pth");


    for thing in chosen_template.read_dir()? {
        let thing = thing?;
        let thing_path = thing.path();

        let thing_name = thing_path.file_name().unwrap();
        let new_path = into.join(thing_name);

        if thing_path.is_dir() {
            fs::create_dir(new_path)?;
        } else {
            fs::copy(thing_path, new_path)?;
        }
    }
    
    // go into templates -> into "idk"
    // take everything inside
    // copy those things into   path



    Ok(())
}




fn remove_contents(path: &Path) -> io::Result<()> {
    for thing in path.read_dir()? {
        let thing = thing?;
        let thing_path = thing.path();

        if thing_path.is_dir() {
            fs::remove_dir_all(thing_path)?;
        } else {
            fs::remove_file(thing_path)?;
        }
    }
    
    Ok(())
}
