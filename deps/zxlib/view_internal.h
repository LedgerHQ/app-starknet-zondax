/*******************************************************************************
*   (c) 2019 Zondax GmbH
*   (c) 2016 Ledger
*
*  Licensed under the Apache License, Version 2.0 (the "License");
*  you may not use this file except in compliance with the License.
*  You may obtain a copy of the License at
*
*      http://www.apache.org/licenses/LICENSE-2.0
*
*  Unless required by applicable law or agreed to in writing, software
*  distributed under the License is distributed on an "AS IS" BASIS,
*  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
*  See the License for the specific language governing permissions and
*  limitations under the License.
********************************************************************************/
#pragma once

#include <stdint.h>
#include <stdbool.h>
#include "coin.h"
#include "zxerror.h"
#include "view.h"

#define CUR_FLOW G_ux.flow_stack[G_ux.stack_count-1]

#define APPROVE_LABEL "APPROVE"
#define REJECT_LABEL "REJECT"

#if defined(TARGET_NANOS)

#define KEY_SIZE 17
#define MESSAGE_SIZE 17

typedef struct NanoSBackend {
  uint8_t key[KEY_SIZE + 1];
  uint8_t value[MESSAGE_SIZE + 1];
  uint8_t value2[MESSAGE_SIZE + 1];
  uintptr_t viewable_size;
  bool expert;
} NanoSBackend;

extern struct NanoSBackend BACKEND_LAZY;

#elif defined (TARGET_NANOX)

#define KEY_SIZE 63
#define MESSAGE_SIZE 4095

typedef struct NanoXBackend {
  uint8_t key[KEY_SIZE + 1];
  uint8_t message[MESSAGE_SIZE + 1];
  uintptr_t viewable_size;
  bool expert;
  bool flow_inside_loop;
} NanoXBackend;

extern struct NanoXBackend BACKEND_LAZY;

#endif

///////////////////////////////////////////////
///////////////////////////////////////////////
///////////////////////////////////////////////
///////////////////////////////////////////////
///////////////////////////////////////////////
///////////////////////////////////////////////
///////////////////////////////////////////////
///////////////////////////////////////////////

void view_idle_show_impl(uint8_t item_idx, char *statusString);
