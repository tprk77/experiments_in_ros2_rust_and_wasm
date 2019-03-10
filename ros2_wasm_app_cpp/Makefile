all:
	mkdir -p build
	em++ -Os -s WASM=1 -s EXPORTED_FUNCTIONS='["_ros_main"]' -s ERROR_ON_UNDEFINED_SYMBOLS=0 \
        -std=c++11 -Isrc src/lib.cpp -o build/ros2_wasm_app_cpp.wasm
	"${BINARYEN_ROOT}/bin/wasm-dis" build/ros2_wasm_app_cpp.wasm -o build/ros2_wasm_app_cpp.wast
	rm build/ros2_wasm_app_cpp.wasm
	sed -i \
        -e "s;_rn_get_default_context;rn_get_default_context;g" \
        -e "s;_rn_create_node;rn_create_node;g" \
        -e "s;_rn_create_publisher;rn_create_publisher;g" \
        -e "s;_rn_std_msg_string_default;rn_std_msg_string_default;g" \
        -e "s;_rn_std_msg_string_set_data;rn_std_msg_string_set_data;g" \
        -e "s;_rn_publish;rn_publish;g" \
        -e "s;_rn_thread_sleep;rn_thread_sleep;g" \
        -e "s;_ros_main;ros_main;g" \
        build/ros2_wasm_app_cpp.wast
	"${BINARYEN_ROOT}/bin/wasm-as" build/ros2_wasm_app_cpp.wast -o build/ros2_wasm_app_cpp.wasm

clean:
	-rm -f build/ros2_wasm_app_cpp.wasm build/ros2_wasm_app_cpp.wast
