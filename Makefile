# Variables
TARGET_DIR := target
BUILD_MODE := debug
TARGET := $(TARGET_DIR)/$(BUILD_MODE)/your_binary_name

# Commands
RUSTC := rustc
CARGO := cargo

# Flags
RUSTFLAGS := -C debuginfo=2 -C target-cpu=native

# Targets
.PHONY: all clean

all: $(TARGET)

$(TARGET): src/*.rs
	$(CARGO) build --$(BUILD_MODE)

clean:
	$(CARGO) clean
