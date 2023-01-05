#!/bin/bash
platform=$(uname -s)
arch=$(uname -m)
from_dir=$PWD
temp_dir="$HOME/.blackhole_temp"
install_dir="/usr/local/bin"
base_url="https://github.com/ruben69695/blackhole/releases/download"
version="v1.1.0"
file="x86_64-unknown-linux-gnu"
extension=".zip"

echo "Blackhole $version installer"

if [ ! -d $temp_dir ]; then
  mkdir $temp_dir
fi

cd $temp_dir

echo "  > ⬇️  Downloading packages..."

if [ $platform == "Darwin" ]; then
  if [ $arch == "a64" ]; then
    file="aarch64-apple-darwin"
  else
    file="x86_64-apple-darwin"
  fi
fi

resource_url="$base_url/$version/$file$extension"
#echo $resource_url

curl -OL --silent $resource_url
unzip -qq $file$extension

echo "  > ⏳ Installing..."
mv "$file/release/blackhole" "$install_dir/"
chmod "u+x" "$install_dir/blackhole"

echo "  > 🧹 Cleaning the house..."
cd $from_dir
rm -R $temp_dir

echo "  > 🍺 Installed!"
echo "  > ⚠️  Reopen your terminal before use the blackhole CLI"
