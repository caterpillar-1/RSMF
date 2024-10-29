BUILD_DIR = build
CONTRACTS_DIR = contracts

$(shell mkdir -p $(BUILD_DIR))

SOLC = ./utils/solc
SOL_SRCS = $(shell find $(CONTRACTS_DIR) -name '*.sol')
SOL_ABIS = $(addprefix $(BUILD_DIR)/,$(patsubst $(CONTRACTS_DIR)/%.sol,%.abi,$(SOL_SRCS)))

$(BUILD_DIR)/%.abi: $(CONTRACTS_DIR)/%.sol $(SOL_SRCS)
	@echo + SOLC '$<' '->' '$@'
	@$(SOLC) '$<' -o $(BUILD_DIR) --bin --abi --overwrite > /dev/null

.PHONY: build clean
.DEFAULT_GOAL := build

build: $(SOL_ABIS)


clean:
	rm -rf $(BUILD_DIR)
