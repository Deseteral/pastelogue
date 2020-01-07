$EXIV2_BUILD_NAME = "exiv2-0.27.2-2017msvc64"

New-Item -Path "." -Name "release/exiv2" -ItemType "directory" -Force
Set-Location .\release
Invoke-WebRequest "https://www.exiv2.org/builds/$EXIV2_BUILD_NAME.zip" -OutFile ".\exiv2.zip"
Expand-Archive ".\exiv2.zip" -DestinationPath ".\exiv2"

Copy-Item ".\exiv2\$EXIV2_BUILD_NAME\bin\exiv2json.exe" -Destination "exiv2\exiv2json.exe"
Copy-Item ".\exiv2\$EXIV2_BUILD_NAME\bin\exiv2.dll" -Destination "exiv2\exiv2.dll"
Copy-Item ".\exiv2\$EXIV2_BUILD_NAME\license.txt" -Destination "exiv2\exiv2_license.txt"

Remove-Item -LiteralPath ".\exiv2\$EXIV2_BUILD_NAME" -Force -Recurse
Remove-Item -LiteralPath ".\exiv2.zip" -Force -Recurse

Set-Location ..
