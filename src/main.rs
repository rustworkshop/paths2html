use std::io::{self, BufRead, Write};

const STDOUT_ERROR: &'static str = "Failed to write to stdout";

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let html_top = r#"
<!DOCTYPE html><html><head><style type="text/css">
*{font-family:monospace;}
.folder {color:blue; cursor:pointer;}
.collapsed li {display:none;}
footer {margin-top:2em;font-size:0.8em;border-top:1px solid #666;padding-top:1em;opacity:0.6;text-align:right;}
</style></head><body>
"#;

    let html_tail = r#"
<script type="text/javascript">
document.querySelectorAll(".folder").forEach((folder) => {
    folder.parentElement.classList.toggle("expando");
    folder.addEventListener("click", (event) => {
        event.target.parentElement.classList.toggle("collapsed");
    });
});
</script>
<footer><div>Generated with <a href="https://github.com/timabell/paths2html">github.com/timabell/paths2html</a>.</div></footer>
</body></html>
"#;

    write!(stdout, "{}", html_top).expect(STDOUT_ERROR);
    write_all(stdin.lock(), &mut stdout);
    write!(stdout, "{}", html_tail).expect(STDOUT_ERROR);
}

// for file paths received on stdout, writes a nested tree of html list nodes
fn write_all<R, W>(reader: R, mut writer: W)
where
    R: BufRead,
    W: Write,
{
    write!(writer, "<ul>").expect(STDOUT_ERROR);
    let mut previous_folder_path = vec![];
    for line in reader.lines() {
        if let Ok(path) = line {
            if path == "" {
                continue; // skip blank lines
            }
            let (folder_path, file) = parse_path(path);
            let match_depth1 = match_depth(&&previous_folder_path, &&folder_path);
            let previous_depth = previous_folder_path.len();
            let previous_folder_levels_to_close = std::cmp::max(0, previous_depth - match_depth1);
            close_folders(&mut writer, previous_folder_levels_to_close);
            open_subfolders(&mut writer, &&folder_path[match_depth1..]);
            write_file(&mut writer, file);
            previous_folder_path = folder_path;
        }
    }
    close_folders(&mut writer, previous_folder_path.len());
    write!(writer, "</ul>").expect(STDOUT_ERROR);
}

fn open_subfolders<W>(writer: &mut W, new_subfolders: &[String])
where
    W: Write,
{
    for folder in new_subfolders.iter() {
        write!(writer, "<li><span class='folder'>{}</span><ul>\n", folder).expect(STDOUT_ERROR);
    }
}

fn write_file<W>(stdout: &mut W, file: String)
where
    W: Write,
{
    write!(stdout, "<li>{}</li>\n", file).expect(STDOUT_ERROR);
}

fn close_folders<W>(writer: &mut W, previous_folder_levels_to_close: usize)
where
    W: Write,
{
    for _ in 0..previous_folder_levels_to_close {
        write!(writer, "</ul>").expect(STDOUT_ERROR);
    }
}

// return folders as vec and filename as str
fn parse_path(path: String) -> (Vec<String>, String) {
    let mut folder: Vec<String> = path
        .split("/")
        .map(|s| s.to_owned()) // clone the vec to avoid keeping a reference to the original string
        .collect();
    let file: String = folder.pop().expect("no file in path");
    return (folder, file);
}

// return number of matching elements (starting at beginning until first mismatch)
fn match_depth(vec1: &[String], vec2: &[String]) -> usize {
    let smallest_length = vec1.len().min(vec2.len());
    vec1.iter()
        .zip(vec2.iter())
        .take(smallest_length)
        .take_while(|(a, b)| a == b)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_write_all() {
        let input = b"x/a\nx/b\nc\n";
        let expected_output = "<ul><li><span class='folder'>x</span><ul>
<li>a</li>
<li>b</li>
</ul><li>c</li>
</ul>";
        // https://stackoverflow.com/questions/28370126/how-can-i-test-stdin-and-stdout/28370712#28370712
        let mut output_buffer = Vec::new();
        write_all(&input[..], &mut output_buffer);
        let actual_output = String::from_utf8(output_buffer).expect("utf8 read failed");
        assert_eq!(expected_output, actual_output);
    }

    #[test_case(&vec![], &vec![], 0)]
    #[test_case(&vec!["a"], &vec!["b"], 0)]
    #[test_case(&vec!["a","b"], &vec!["x","y"], 0)]
    #[test_case(&vec!["a","b"], &vec!["a","x"], 1)]
    #[test_case(&vec!["a","b"], &vec!["a","b"], 2)]
    #[test_case(&vec!["a","b", "c"], &vec!["a","b"], 2)]
    fn test_match_depth(vec1: &[&str], vec2: &[&str], expected: usize) {
        let vec1_strings: Vec<String> = vec1
            .iter()
            .map(|s| s.to_string()) // clone the vec to avoid keeping a reference to the original string
            .collect();
        let vec2_strings: Vec<String> = vec2
            .iter()
            .map(|s| s.to_string()) // clone the vec to avoid keeping a reference to the original string
            .collect();
        assert_eq!(expected, match_depth(&vec1_strings, &vec2_strings));
    }

    #[test_case("a.txt", &vec![], "a.txt")]
    #[test_case("a/b/c.txt", &vec!["a","b"], "c.txt")]
    fn test_parse_path(input_path: &str, expected_folders: &[&str], expected_file: &str) {
        let (actual_folders, actual_file) = parse_path(input_path.to_string());
        assert_eq!(expected_file, actual_file);
        assert_eq!(expected_folders, actual_folders);
    }
}
