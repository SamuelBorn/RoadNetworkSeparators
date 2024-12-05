if [ ! -d "KaHIP/deploy" ]; then
    ./KaHIP/compile_withcmake.sh
fi

(
    mkdir -p build
    cd build
    cmake ..
    make
)
