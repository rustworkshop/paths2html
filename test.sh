echo "abc/foo.txt
abc/bar.txt
abc/def/xx.txt
hurrah.txt" | cargo run | tee test.html && xdg-open test.html
