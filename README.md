# X Labs Updater
## WIP

Latest release
  - [Linux](https://github.com/mxve/xlabs-updater/releases/latest/download/xlabs-updater-x86_64-unknown-linux-gnu.tar.gz)
    - ```https://github.com/mxve/xlabs-updater/releases/latest/download/xlabs-updater-x86_64-unknown-linux-gnu.tar.gz```
  - [Windows](https://github.com/mxve/xlabs-updater/releases/latest/download/xlabs-updater-x86_64-pc-windows-msvc.zip)
    - ```https://github.com/mxve/xlabs-updater/releases/latest/download/xlabs-updater-x86_64-pc-windows-msvc.zip```
  - [MacOS](https://github.com/mxve/xlabs-updater/releases/latest/download/xlabs-updater-x86_64-apple-darwin.tar.gz) (untested)
    - ```https://github.com/mxve/xlabs-updater/releases/latest/download/xlabs-updater-x86_64-apple-darwin.tar.gz```

### Arguments
- ```-d, --directory <path>```
  - Install directory
  - Default: "xlabs"
- ```-l, --launcher```
  - Download launcher files
- ```--iw4x-path```
  - Set IW4x game path
  - Needs to be set at least once to install/update IW4x rawfiles. Last path is saved to xlabs-updater.json inside the install dir (-d) and will be used if no path is specified.
- ```--dev```
  - Update from dev branch