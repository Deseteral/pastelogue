# pastelogue
Automatically catalogue your photo collection.

## Dependencies
To extract Exif metadata from files pastelogue uses Exiv2 library. To setup this dependency run
```shell
./scripts/macos_setup_exiv2.sh
```
or if you're on Windows
```powershell
./scripts/windows_setup_exiv2.ps1
```

## Making release package
For MacOS run
```shell
./scripts/windows_make_release.sh
```
or if you're on Windows
```powershell
./scripts/windows_make_release.ps1
```

Packages with binaries built for specific platform are in `release` directory.

## License
This project is licensed under the [MIT license](LICENSE).
