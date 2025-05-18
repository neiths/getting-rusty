for i in $(seq 11 20); do
  folder="day$i"
  mkdir "$folder"
  echo "# 🚀 Day $i - Learning Rust" > "$folder/README.md"
done
