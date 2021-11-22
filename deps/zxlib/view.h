/*******************************************************************************
*   (c) 2018-2020 Zondax GmbH
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
#include "coin.h"
#include "zxerror.h"

#if defined(LEDGER_SPECIFIC)
#include "bolos_target.h"
#if defined(BOLOS_SDK)
#include "os.h"
#include "cx.h"
#endif
#endif

typedef zxerr_t (*viewfunc_getNumItems_t)(uint8_t *num_items);

typedef zxerr_t (*viewfunc_getItem_t)(int8_t displayIdx,
                                      char *outKey, uint16_t outKeyLen,
                                      char *outVal, uint16_t outValLen,
                                      uint8_t pageIdx, uint8_t *pageCount);

typedef void (*viewfunc_accept_t)();

#ifdef APP_SECRET_MODE_ENABLED
zxerr_t secret_enabled();
#endif

void zemu_log_stack(char *ctx);

/// view_init (initializes UI)
void view_init();

/// view_idle_show (idle view - main menu + status)
void view_idle_show(uint8_t item_idx, char *statusString);
