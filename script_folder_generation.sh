for i in $(seq 3 10); do
  folder="day$i"
  mkdir "$folder"
  echo "# 🚀 Day $i - Learning Rust" > "$folder/README.md"
done
