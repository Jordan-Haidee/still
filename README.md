# Still
Still is a simple process alarm. It make specified process sleep now, and wake up
after a period of time.

![banner](assets/banner.png)

## Usage
```shell
still $pid $duration
```
some duration examples:
```
still 19604 30       # make process 19604 sleep 30 seconds
still 19604 30s      # make process 19604 sleep 30 seconds
still 19604 30m      # make process 19604 sleep 30 minites
still 19604 30h      # make process 19604 sleep 30 hours
```
## Install
### Binaries
Just download from releases. On Windows, two binaries must be placed at the some folder.
### Build from source
#### Windows
```powershell
./build.ps1
```
#### Linux
```bash
bash ./build.sh
```

