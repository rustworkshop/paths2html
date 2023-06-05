use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let html_top = "
<!DOCTYPE html><html><head><style type=\"text/css\">
     * { font-family: monospace; }
     .folder {color: blue; cursor: pointer;}
     .collapsed li { display: none; }
</style></head><body>

<ul>
    <li><span class=\"folder\">foo/</span>
       <ul>
           <li><span class=\"folder\">bar/</span>
             <ul>
               <li>a.txt</li>
               <li>b.txt</li>
             </ul>
           </li>
           <li>c.txt</li>
       </ul>
    <li>hello.txt</li>
</ul>

<ul>
";
    let html_tail = "
</ul><script type=\"text/javascript\">
     document.querySelectorAll(\".folder\").forEach((folder) => {
         folder.parentElement.classList.toggle(\"expando\");
         folder.addEventListener(\"click\", (event) => {
             event.target.parentElement.classList.toggle(\"collapsed\");
         });
     });
</script></body></html>
";
    write!(stdout, "{}", html_top).expect("Failed to write to stdout");

    for line in stdin.lock().lines() {
        if let Ok(path) = line {
            write!(stdout, "  <li>{}</li>\n", path).expect("Failed to write to stdout");
        }
    }

    write!(stdout, "{}", html_tail).expect("Failed to write to stdout");
}
