$base_url = "https://github.com/ruben69695/blackhole/releases/download"
$version = "v1.1.0"
$file = "x86_64-pc-windows-gnu"
$extension = ".zip"
$from_dir = $PWD.Path

$resource_url = "$base_url/$version/$file$extension"

Write-Host "Blackhole $version installer"

if (-NOT (Test-Path C:\blackhole_temp)) {
    New-Item -Path C:\blackhole_temp -ItemType Directory
}

Set-Location C:\blackhole_temp

Write-Host "  > ⬇️  Downloading packages..."
Invoke-WebRequest -Uri $resource_url -OutFile $file$extension
Expand-Archive -Path $file$extension -DestinationPath $file

Write-Host "  > ⏳ Installing..."
Move-Item '$file\release\blackhole' C:\Windows\System32

Write-Host "  > 🧹 Cleaning the house..."
Remove-Item C:\blackhole_temp -Recurse
Set-Location $from_dir

Write-Host "  > 🍺 Installed!"
Write-Host "  > ⚠️  Reopen your terminal before use the blackhole CLI"