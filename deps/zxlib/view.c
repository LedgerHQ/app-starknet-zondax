/*******************************************************************************
*   (c) 2018, 2019 Zondax GmbH
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

#include "view.h"
#include "coin.h"
#include "view_internal.h"

#include "zxmacros.h"
#include "actions.h"
#include "ux.h"
#include "bagl.h"
#include "view_templates.h"
#include "app_mode.h"
#include "zxerror.h"

#include <string.h>
#include <stdio.h>
#include <stdbool.h>

///////////////////////////////////
// General

void io_seproxyhal_display(const bagl_element_t *element) {
    io_seproxyhal_display_default((bagl_element_t *) element);
}
void view_init(void) {
    UX_INIT();
}

void view_idle_show(uint8_t item_idx, char *statusString) {
    view_idle_show_impl(item_idx, statusString);
}
