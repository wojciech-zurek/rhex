# rhex

Another command-line hex viewer

### Install (linux 64bit)

```
sudo wget https://github.com/wojciech-zurek/rhex/releases/download/v0.1.0/rhex -O /usr/local/bin/rhex
sudo chmod +x /usr/local/bin/rhex
```

### Install (aarch64-linux-android, termux, tested on Gemini PDA)

```
wget https://github.com/wojciech-zurek/rhex/releases/download/v0.1.0/rhex-aarch64-linux-android -O $HOME/../usr/bin/rhex
chmod +x $HOME/../usr/bin/rhex
```

### Build

```bash
./build.sh
```

### Cross build

```bash
./cross_build.sh
```

### Example usage

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

```
Termux example usage
```

![rhex](shot/shot9.png "rhex")