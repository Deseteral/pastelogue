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
./scripts/macos_make_release.sh
```
or if you're on Windows
```powershell
./scripts/windows_make_release.ps1
```

Packages with binaries built for specific platform are in `release` directory.

## Server
### Processing
Request:
```json
{
  "action": "START_PROCESSING",
  "args": {
    "path": "/Some/user/path/to/photos/directory"
  }
}
```

Response:
```json
{
  "id": "PROCESSING_STARTED",
  "payload": null
}
{
  "id": "PROCESSING_PROGRESS",
  "payload": {
    "progress": 12,
    "total": 674
  }
}
{
  "id": "PROCESSING_FINISHED",
  "payload": null
}
```

### Read EXIF data from single image
Request:
```json
{
  "action": "READ_EXIF_DATA",
  "args": {
    "path": "/Some/user/path/to/single_photo.jpeg"
  }
}
```

Response:
```json
{
  "id": "EXIF_DATA",
  "payload": {
    "exif_data": { ... }
  }
}
```

### Error
Response:
```json
{
    "id": "ERROR",
    "payload": {
        "messages": [
            "Some error message",
            "Other error message"
        ]
    }
}
```

## License
This project is licensed under the [MIT license](LICENSE).
