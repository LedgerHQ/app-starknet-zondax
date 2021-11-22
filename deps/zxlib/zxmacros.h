/*******************************************************************************
*   (c) 2018 Zondax GmbH
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

#ifdef __cplusplus
extern "C" {
#endif

#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include "string.h"

#define __Z_INLINE inline __attribute__((always_inline)) static
#define NV_ALIGN __attribute__ ((aligned(64)))

#if defined(LEDGER_SPECIFIC)
#include "bolos_target.h"
#endif

#if defined (TARGET_NANOS) || defined(TARGET_NANOX)
#include "zxmacros_ledger.h"
#else
#error "Not supported"
#endif

#ifndef UNUSED
#define UNUSED(x) (void)x
#endif


#ifdef __cplusplus
}
#endif
