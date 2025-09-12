CC = arm-none-eabi-gcc
CXX = arm-none-eabi-g++
BUILD_DIR = target
SIM_DIR = epsilon_simulator
NWLINK = npx --yes -- nwlink@0.0.19
LINK_GC = 1
LTO = 1
NAME = grapher-3d

define object_for
$(addprefix $(BUILD_DIR)/,$(addsuffix .o,$(basename $(1))))
endef
# what

src = $(addprefix src/,\
	main.cpp \
	menu.cpp \
	pompeiu.cpp \
	graph_gen.cpp \
	renderer.cpp \
	graph_display.cpp \
)

CPPFLAGS = -std=c++11 -fno-exceptions
CPPFLAGS += -Os -Wall
CPPFLAGS += $(shell $(NWLINK) eadk-cflags-device)  # what does this do?
CFLAGS = -std=c99
CFLAGS += $(shell $(NWLINK) eadk-cflags-device)
CFLAGS += -Os -Wall
CFLAGS += -ggdb
LDFLAGS = -Wl,--relocatable
LDFLAGS += -nostartfiles
LDFLAGS += --specs=nano.specs

ifeq ($(LINK_GC),1)
CFLAGS += -fdata-sections -ffunction-sections
CPPFLAGS += -fdata-sections -ffunction-sections
LDFLAGS += -Wl,-e,main -Wl,-u,eadk_app_name -Wl,-u,eadk_app_icon -Wl,-u,eadk_api_level
LDFLAGS += -Wl,--gc-sections
endif

ifeq ($(LTO),1)
CFLAGS += -flto -fno-fat-lto-objects
CFLAGS += -fwhole-program
CFLAGS += -fvisibility=internal
CPPFLAGS += -flto -fno-fat-lto-objects
CPPFLAGS += -fwhole-program
CPPFLAGS += -fvisibility=internal
LDFLAGS += -flinker-output=nolto-rel
endif


# NWA (from NW)
.PHONY: build
build: $(BUILD_DIR)/$(NAME).bin

.PHONY: run
run: $(BUILD_DIR)/$(NAME).nwa
	@ echo "INSTALL $<"
	@ $(NWLINK) install-nwa $<

$(BUILD_DIR)/%.bin: $(BUILD_DIR)/%.nwa
	@ echo "BIN     $@"
	@ $(NWLINK) nwa-bin $< $@

$(BUILD_DIR)/$(NAME).nwa: $(call object_for,$(src)) $(BUILD_DIR)/icon.o
	@ echo "LD      $@"
	@ $(CC) $(CPPFLAGS) $(LDFLAGS) $^ -o $@

$(addprefix $(BUILD_DIR)/,%.o): %.c | $(BUILD_DIR)
	@ echo "C       $^"
	@ $(CC) $(CFLAGS)$(SFLAGS) -c $^ -o $@

$(addprefix $(BUILD_DIR)/,%.o): %.cpp | $(BUILD_DIR)
	@ echo "CXX     $^"
	@ $(CXX) $(CPPFLAGS) $(SFLAGS) -c $^ -o $@

$(BUILD_DIR)/icon.o: assets/icon.png
	@ echo "ICON    $<"
	@ $(NWLINK) png-icon-o $< $@

.PRECIOUS: $(BUILD_DIR)
$(BUILD_DIR):
	@ mkdir -p $@/src

.PHONY: clean
clean:
	@ echo "CLEAN"
	@ rm -rf $(BUILD_DIR)


# Simulator
.PHONY: sim
sim: $(BUILD_DIR)/$(NAME).bin | $(SIM_DIR)/output/release/simulator/linux/epsilon.bin 
	@ echo "SIM      "
	./$(SIM_DIR)/output/release/simulator/linux/epsilon.bin --nwb $(BUILD_DIR)/$(NAME).bin

$(SIM_DIR)/output/release/simulator/linux/epsilon.bin: | $(SIM_DIR)
	@ echo "BUILD   sim"
	@ cd $(SIM_DIR) && make PLATFORM=simulator -j 1
# multiple jobs??

.PHONY: clone
clone: $(SIM_DIR)

.PRECIOUS: $(SIM_DIR)
$(SIM_DIR):
	@ echo "CLONE   sim"
	@ git clone https://github.com/numworks/epsilon.git epsilon_simulator -b version-20

.PHONY: clean-sim
clean-sim:
	@ echo "CLEAN   sim"
	@ rm -rf $(SIM_DIR)
