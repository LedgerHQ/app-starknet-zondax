/*******************************************************************************
*   (c) 2019 Zondax GmbH
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
#include <os_io_seproxyhal.h>
#include "inttypes.h"
#include "zxmacros.h"

#define APDU_CODE_DATA_INVALID              0x6984
#define APDU_CODE_COMMAND_NOT_ALLOWED       0x6986

__Z_INLINE void set_code(uint8_t *buffer, uint8_t offset, uint16_t value) {
    *(buffer + offset) = (uint8_t) (value >> 8);
    *(buffer + offset + 1) = (uint8_t) (value & 0xFF);
}

__Z_INLINE void app_reject() {
    set_code(G_io_apdu_buffer, 0, APDU_CODE_COMMAND_NOT_ALLOWED);
    io_exchange(CHANNEL_APDU | IO_RETURN_AFTER_TX, 2);
}

__Z_INLINE void app_reply_error() {
    set_code(G_io_apdu_buffer, 0, APDU_CODE_DATA_INVALID);
    io_exchange(CHANNEL_APDU | IO_RETURN_AFTER_TX, 2);
}
