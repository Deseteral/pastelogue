$EXIV2_BUILD_NAME = "exiv2-0.27.2-2017msvc64"

Set-Location .\build
Invoke-WebRequest "https://www.exiv2.org/builds/$EXIV2_BUILD_NAME.zip" -OutFile ".\exiv2.zip"
New-Item -Path "." -Name "exiv2" -ItemType "directory" -Force
Expand-Archive ".\exiv2.zip" -DestinationPath ".\exiv2"

Copy-Item ".\exiv2\$EXIV2_BUILD_NAME\bin\exiv2json.exe" -Destination "exiv2\exiv2json"
Copy-Item ".\exiv2\$EXIV2_BUILD_NAME\bin\exiv2.dll" -Destination "exiv2\exiv2.dll"

Remove-Item -LiteralPath ".\exiv2\$EXIV2_BUILD_NAME" -Force -Recurse
Remove-Item -LiteralPath ".\exiv2.zip" -Force -Recurse

Set-Location ..
