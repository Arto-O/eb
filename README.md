# eb

**eb** aims to combine the functionalities of the Unix command-line programs `ls` and `cat` as well as improve upon them. The name comes from the initials of [exa](https://github.com/ogham/exa) and [bat](https://github.com/sharkdp/bat) which are popular enhancements of `ls` and `cat` respectively.

## What's the point

I do almost everything I can through the command-line. I often find myself first using `exa` to find a file and then printing it out with `bat`. However, I often accidentally try to use `exa` with the file path. This slows down my workflow considerably. Having one program for both should allow me to switch from listing to printing more quickly and easily.

## Plans

The usage of **eb** will be similar to the usage of `exa` or `bat`. By default, if the provided path is a directory, **eb** will list the files and directories within it. If the path is a file, **eb** will print out its contents.

After the core functionality is complete, it would be cool to have additional features, such as printing out smaller files' contents within the directory listing.