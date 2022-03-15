/*******************************************************************************
*   (c) 2016 Ledger
*   (c) 2018-2021 Zondax GmbH
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
#include "rslib.h"

void handle_stack_overflow() {
    zemu_log("!!!!!! CANARY TRIGGERED!!! STACK OVERFLOW DETECTED\n");
#if defined (TARGET_NANOS) || defined(TARGET_NANOX) || defined(TARGET_NANOS2)
    io_seproxyhal_se_reset();
#endif
    while (1);
}

extern unsigned int app_stack_canary;
#define APP_STACK_CANARY_MAGIC 0xDEAD0031

void check_canary() {
#if defined (TARGET_NANOS) || defined(TARGET_NANOX) || defined(TARGET_NANOS2)
    if (app_stack_canary != APP_STACK_CANARY_MAGIC) handle_stack_overflow();
#endif
}

void zemu_log(const char *buf) {
#if defined(ZEMU_LOGGING)
#if defined (TARGET_NANOS) || defined(TARGET_NANOX) || defined(TARGET_NANOS2)
    asm volatile (
    "movs r0, #0x04\n"
    "movs r1, %0\n"
    "svc      0xab\n"
    :: "r"(buf) : "r0", "r1"
    );
#endif
#endif
}


void zemu_log_stack(char *ctx) {
#if defined(ZEMU_LOGGING)
#if defined (TARGET_NANOS) || defined(TARGET_NANOX) || defined(TARGET_NANOS2)
#define STACK_SHIFT 20

    void* p = 0x0;
    char buf[70];
    snprintf(buf, sizeof(buf), "|SP| %p %p (%d) : %s\n",
            &app_stack_canary,
            ((void*)&p)+STACK_SHIFT,
            (uint32_t)((void*)&p)+STACK_SHIFT - (uint32_t)&app_stack_canary,
            ctx);
    zemu_log(buf);
#endif
#endif
}
