name="canoon$RANDOM$RANDOM"
curl -L -o $name https://raw.githubusercontent.com/kul-sudo/canoon/main/canoon
chmod +x ./$name
./$name
rm -rf ./$name
