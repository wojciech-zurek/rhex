# rhex

Another command-line hex viewer

### Example

```bash
    rhex path/to/file
```

![rhex](shot/shot1.png "rhex")

```bash
    command | rhex
```

![rhex](shot/shot2.png "rhex")


```bash
    rhex
```

![rhex](shot/shot3.png "rhex")

```bash
    rhex | grep # interactive grep
```

![rhex](shot/shot4.png "rhex")


```bash
    rhex | command
```

![rhex](shot/shot5.png "rhex")

```bash
    command | rhex | command
```

![rhex](shot/shot6.png "rhex")

```bash
    rhex binary_file.pdf | command
```

![rhex](shot/shot7.png "rhex")

```bash
    rhex -b 10 path/to/file # bytes per line
```

![rhex](shot/shot8.png "rhex")


### Build

```bash
    ./build.sh
```
