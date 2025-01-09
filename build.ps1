cargo build --release --bins
rm bins -r
mkdir bins 
mv .\target\release\core.exe bins
mv .\target\release\still.exe bins
