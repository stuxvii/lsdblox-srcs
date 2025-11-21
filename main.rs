// Hi, It seems liek ur looking thhhru the source.Thanks for downloading!!
// Lincense: GPL V3!! IF YOU'RE USING THIS PROGRAM PLS INCLUDE IT!!!! YES!
// ALSO YEAH THE PROGRAM IS VERY SIMPL!! AS LONG AS YOU HAVE BASIC ENGLISH
// UNDERSTANDING IT SHOULD BE EASY TO READ AND UNDERSTAND EVERYTHING!!!!!!
use std::env;
use std::io::Write;
use std::fs;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: {} filename.obj", args[0]);
        println!("to output to terminal instead of saving to a file, use the -c option.");
        return Ok(());
    }

    let path = &args[1];
    let contents = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Couldn't read file {}: {}", path, e);
            return Err(e);
        }
    };

    let mut vertices: Vec<Vec<f32>> = Vec::new();

    for line in contents.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.starts_with('v') && trimmed_line.chars().nth(1) == Some(' ') {
            let components: Vec<&str> = trimmed_line
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect();

            if components.len() >= 4 {
                if let (Ok(x), Ok(y), Ok(z)) = (
                    components[1].parse::<f32>(),
                    components[2].parse::<f32>(),
                    components[3].parse::<f32>()
                ) {
                    vertices.push(vec![x, y, z]);
                } else {
                    eprintln!("WARNING! Error parsing line {}", trimmed_line);
                }
            }
        }
    }
    let count_header = vertices.len() / 9;
    let mut count_written = 0;

    if args.len() > 2 && args[2] == "-c" {
        println!("version 1.00");
        println!("{}", count_header);

        for point in vertices {
            count_written += 1;
            print!("[{},{},{}]", point[0], point[1], point[2]);
        }

        print!("\n");
        count_written /= 9;
    } else {
        let filename = format!("{}.mesh", &args[1]);
        let mut file = File::create(&filename)?;

        writeln!(file, "version 1.00")?;
        writeln!(file, "{}", count_header)?;

        for point in vertices {
            count_written += 1;
            write!(file, "[{},{},{}]", point[0], point[1], point[2])?;
        }

        write!(file, "\n")?;
        println!("Mesh written to file {}.", filename);
        count_written /= 9;
    }

    if count_written != count_header {
        println!("Warning!! The number of written points ({}) does not match the header count ({}). This makes it so that your file is kinda just unusable for rblx. This ideally shouldn't happen with a good OBJ file so try re exporting the one you have with sm like blender. Sorry for the inconvenience!!!", count_written, count_header);
    }

    Ok(()) // la la la la la la
}
