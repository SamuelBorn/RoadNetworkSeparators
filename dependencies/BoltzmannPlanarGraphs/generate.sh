(
    cd ./build/classes
    echo 100000 | java -Xss8m boltzmannplanargraphs.Main
)
mv ListEdges.txt "out/$(date +"%Y-%m-%d_%H:%M:%S").txt"
