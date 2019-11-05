$PKG_VERSION = (Get-Content -Path ".\Cargo.toml" | Select-String -AllMatches -Pattern 'version = "(.*)"').Matches.Groups[1].Value
$DIST_NAME = "pastelogue_${PKG_VERSION}_Windows"
$DIST_PATH = ".\dist\${DIST_NAME}"

New-Item -Path "." -Name $DIST_PATH -ItemType "directory" -Force

Copy-Item ".\target\release\pastelogue.exe" -Destination $DIST_PATH
Copy-Item ".\build\exiv2\*" -Destination $DIST_PATH

Compress-Archive -Path "$DIST_PATH\*" -DestinationPath "${DIST_PATH}.zip" -Force

Remove-Item -LiteralPath "$DIST_PATH" -Force -Recurse
