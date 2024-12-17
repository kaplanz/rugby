# --------------------
# --    Project     --
# --------------------

# Package
CRATE ?= rugby


# --------------------
# --  Directories   --
# --------------------

# Build artifacts
BUILD ?= ./build

# Target directories
FFI = $(BUILD)/ffi
PKG = ./pkg


# --------------------
# --   Artifacts    --
# --------------------

# Target platforms
DEVS = aarch64-apple-ios aarch64-apple-ios-sim

# Static libraries
LIBS = $(DEVS:%=$(BUILD)/%/release/lib$(CRATE).a)

# Xcode framework
XCFW = $(PKG)/RugbyKit.xcframework


# --------------------
# --    Targets     --
# --------------------

# Make targets
.PHONY: all
all: build

# Build all
.PHONY: build
build: lib ffi pkg

# Clean all
.PHONY: clean
clean:
	rm -rfv $(BUILD)

# Static libraries
.PHONY: lib
lib: $(LIBS)

$(BUILD)/%/release/lib$(CRATE).a:
	cargo build --lib --release --target-dir $(BUILD) --target $*

# Swift bindings
.PHONY: ffi
ffi: $(FFI)

$(FFI): $(LIBS)
	cargo build --bin uniffi
	cargo run -q --bin uniffi --                  \
		$(firstword $(LIBS))                  \
		$(FFI)                                \
		--module-name $(CRATE)FFI             \
		--swift-sources --headers --modulemap \
		--modulemap-filename module.modulemap

# Xcode framework
.PHONY: pkg
pkg: $(XCFW)
$(XCFW): $(LIBS) $(FFI)
	xcodebuild -create-xcframework -output $(XCFW) \
		$(foreach LIB,$(LIBS),-library $(LIB) -headers $(FFI))
