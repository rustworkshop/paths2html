echo "abc/x-1.txt\nabc/x-2.txt\nabc/def/ghi/x-2.txt\nabc/x-4.txt\nhurrah.txt" | cargo run | tee test.html && xdg-open test.html
