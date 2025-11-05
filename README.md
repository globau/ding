# ding

A simple command-line utility that plays sounds when commands complete.

## Usage

```
ding [command]
```

**Platform support:** macOS or Linux only.

## Description

Without a command, `ding` plays a happy ding sound.

With a command, `ding` will execute that command and play a ding if the command
completed without error, otherwise an error sound will be played.

If the command took more than five seconds to execute, the elapsed time will be
shown.

## Credits

Sounds are sourced from [freesound.org](https://freesound.org):
- [ding.ogg](http://freesound.org/people/pan14/sounds/263133/) by pan14
- [error.ogg](http://freesound.org/people/rhodesmas/sounds/342756/) by rhodesmas

## Licence

MIT Licensed.

---

byron@glob.au
