# Every so on
A rust script that updates your wallpaper every so on (every 1 hour, to be precise) made for Wayland.



## installation
- 1. Install [swaybg](https://github.com/swaywm/swaybg)
- 2. Create a directory in your user root directory. ex: `my_linux_user/wallpapers`
  3. Fill the directory `my_linux_user/wallpapers` with the wallpaper image files you want the script to pick from
- 4. Clone this repo
- 5. In the directory of every-so-on*this repo), run
``` bash
cargo build --release
```
- 6. Execute the program by running the executable found in `target/release/every-so-on`
- 7. Now you can find a way in your distribution to run this script every time you start your desktop environment. 
