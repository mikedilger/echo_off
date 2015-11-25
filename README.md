# echo_off

This is a rust library for adjusting the terminal echo property.  For instance, when
reading a password from the user you may want to turn echo off, and then back on again
afterwards.

Not tested on any machines but my own gnu-linux ones.

Turn off echo with:

```.rust
    let mut termios: Termios = Termios::new();
    termios.echo_off();
```

Then later:
```.rust
    termios.echo_on();
```
