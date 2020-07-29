OUT_DIR = ${CURDIR}/bin
SRC_DIR = ${CURDIR}/src


WASM_PACK_FLAGS = build --out-dir $(OUT_DIR) --target web
WASM_PACK = wasm-pack


dev: static_files
	cd $(SRC_DIR)/core; $(WASM_PACK) $(WASM_PACK_FLAGS) --dev

	rm $(OUT_DIR)/core_bg.d.ts
	rm $(OUT_DIR)/core.d.ts
	rm $(OUT_DIR)/package.json


release: static_files
	cd $(SRC_DIR)/core; $(WASM_PACK) $(WASM_PACK_FLAGS) --release

	rm $(OUT_DIR)/core_bg.d.ts
	rm $(OUT_DIR)/core.d.ts
	rm $(OUT_DIR)/package.json


static_files: 
	cp src/index.html $(OUT_DIR)/index.html
	cp src/style.css $(OUT_DIR)/style.css


fmt:
	cd $(SRC_DIR)/core; cargo fmt
