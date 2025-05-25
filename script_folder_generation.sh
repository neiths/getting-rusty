for i in $(seq 21 30); do
  folder="day$i"
  mkdir "$folder"
  echo "# 🚀 Day $i - Learning Rust" > "$folder/README.md"
done
