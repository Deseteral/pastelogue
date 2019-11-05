$PKG_VERSION = (Get-Content -Path ".\Cargo.toml" | Select-String -AllMatches -Pattern 'version = "(.*)"').Matches.Groups[1].Value
$RELEASE_NAME = "pastelogue_${PKG_VERSION}_Windows"
$RELEASE_PATH = ".\release\${RELEASE_NAME}"

New-Item -Path "." -Name $RELEASE_PATH -ItemType "directory" -Force

Copy-Item ".\target\release\pastelogue.exe" -Destination $RELEASE_PATH
Copy-Item ".\build\exiv2\*" -Destination $RELEASE_PATH

Compress-Archive -Path "$RELEASE_PATH\*" -DestinationPath "${RELEASE_PATH}.zip" -Force

Remove-Item -LiteralPath "$RELEASE_PATH" -Force -Recurse
