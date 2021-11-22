#*******************************************************************************
#*   (c) 2019 Zondax GmbH
#*
#*  Licensed under the Apache License, Version 2.0 (the "License");
#*  you may not use this file except in compliance with the License.
#*  You may obtain a copy of the License at
#*
#*      http://www.apache.org/licenses/LICENSE-2.0
#*
#*  Unless required by applicable law or agreed to in writing, software
#*  distributed under the License is distributed on an "AS IS" BASIS,
#*  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#*  See the License for the specific language governing permissions and
#*  limitations under the License.
#********************************************************************************

# We use BOLOS_SDK to determine the development environment that is being used
# BOLOS_SDK IS  DEFINED	 	We use the plain Makefile for Ledger
# BOLOS_SDK NOT DEFINED		We use a containerized build approach

TESTS_JS_PACKAGE = "@zondax/ledger-template-app"
TESTS_JS_DIR = $(CURDIR)/js

ifeq ($(BOLOS_SDK),)
	include $(CURDIR)/deps/dockerized_build.mk

build:
	$(MAKE)
.PHONY: build

lint:
	cargo fmt
.PHONY: lint

clippy:
	cargo clippy --all-targets
.PHONY: clippy


.PHONY: zemu_test
zemu_test:
	cd $(TESTS_ZEMU_DIR) && yarn test$(COIN)

.PHONY: zemu_debug
zemu_debug:
	cd $(TESTS_ZEMU_DIR) && yarn run debug

.PHONY: rust_test
rust_test:
	cargo test

test_all:
	make rust_test
	make zemu_install
	make clean_build
	make build
	make zemu_test

.PHONY: fuzz clean_fuzz
fuzz:
	cd hfuzz && cargo hfuzz run apdu

clean_fuzz:
	cd hfuzz && cargo hfuzz clean

else

default:
	$(MAKE) -C app

%:
	$(info "Calling app Makefile for target $@")
	COIN=$(COIN) $(MAKE) -C app $@

endif
