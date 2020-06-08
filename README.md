# pastelogue
Automatically catalogue your photo collection.

## Cataloging algorithm
Every media file inside the library directory has to be in correct location with correct name.

### Directory structure scheme
```
<library>/<year>/<month>/<day>
```
where
- `year` is in four digit notation (so `1996` not `96`)
- `month` and `day` are numbers (with leading zero for values from 1 to 9)

### Filename scheme
```
<year>-<month>-<day>_<hour>-<minute>-<second><_optional_counter>.<file_extension>
```
where
- `year`, `month`, `day` are the same as in directory name
- `hour` is number from 00 to 23 (24h format, so 19:00 not 7pm)
- `hour`, `minute`, `second` are numbers with leading zero for values from 1 to 9
- `optional_couter` is for cases when there was more then one photo taken in the same second. When that is the case, first photo doesn't have a counter, and second one has `_1` suffix

### Example
Given two photos that were taken on June 21st 2019 18:13:37 the paths should look like that:
```
/2019/06/21/2019-06-21_18-13-37.jpeg
/2019/06/21/2019-06-21_18-13-37_1.jpeg
```

## Development
### Dependencies
To extract Exif metadata from files pastelogue uses Exiv2 library. To setup this dependency run
```shell
./scripts/macos_setup_exiv2.sh
```
or if you're on Windows
```powershell
./scripts/windows_setup_exiv2.ps1
```

### Making release package
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
When the server starts it will emit the `READY` event:
```json
{
  "id": "READY",
  "payload": {
    "version": "0.4.0"
  }
}
```

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
    "progress": {
      "current": 231,
      "total": 674
    },
    "file": {
      "input": {
        "path": "/Some/user/path/to/single_photo.jpeg"
      },
      "output": {
        "path": "/Some/user/path/to/single_photo_with_correct_path_and_name.jpeg"
      }
    },
    "metadata": {
        "date": "2020-06-08T20:02:24"
    }
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
