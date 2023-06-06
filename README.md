Pipe a list of file paths to this program and it will generate an html file with the folders as clickable lists that will collapse when you click on them.

This makes it easy to take a massive file list and collapse things that you've looked at or that aren't interesting.


This is going to be used to be able to more quickly eyeball md5deep verification output.

See
<https://timwise.co.uk/2022/03/02/detecting-bit-rot-with-md5deep/> and
<https://gist.github.com/timabell/f70f34f8933b2abaf42789f8afdbd7d5>

Usage:

```bash
grep "Known file not used" ~/Documents/hashdeep-checksums-verification.txt \
  | sed 's/:.*//'| paths2html \
  | tee tmp.html \
  && xdg-open tmp.html
```
