# --------------------
# --    Project     --
# --------------------

# Package
CRATE ?= rugby
XCODE ?= RugbyKit


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
DEVS = aarch64-apple-darwin aarch64-apple-ios aarch64-apple-ios-sim

# Static libraries
LIBS = $(DEVS:%=$(BUILD)/%/release/lib$(CRATE).a)

# Swift sources
SRCS = $(PKG)/Sources/$(XCODE)

# Xcode framework
XFWK = $(PKG)/$(XCODE).xcframework


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
clean: clean-pkg
	@rm -rfv $(BUILD)

# Clean packages
.PHONY: clean-pkg
clean-pkg:
	@rm -rfv $(SRCS) $(XFWK)

# Static libraries
.PHONY: lib
lib: $(LIBS)

$(BUILD)/%/release/lib$(CRATE).a:
	@cargo build --lib --release --target-dir $(BUILD) --target $*

# Swift bindings
.PHONY: ffi
ffi: $(FFI)

$(FFI): $(LIBS)
	@cargo build --quiet --bin uniffi
	@cargo run -q --bin uniffi --                 \
		$(firstword $(LIBS))                  \
		$(FFI)                                \
		--module-name $(CRATE)FFI             \
		--modulemap-filename module.modulemap \
		--headers --modulemap
	@cargo run -q --bin uniffi --                 \
		$(firstword $(LIBS))                  \
		$(SRCS)                               \
		--module-name $(CRATE)FFI             \
		--swift-sources

# Xcode framework
.PHONY: pkg
pkg: $(XFWK)
$(XFWK): $(LIBS) $(FFI)
	@rm -rf $(XFWK)
	@xcodebuild -create-xcframework -output $(XFWK) \
		$(foreach LIB,$(LIBS),-library $(LIB) -headers $(FFI))
