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

#include "app_mode.h"
#include "view.h"
#include "view_internal.h"
#include "ux.h"
#include "bagl.h"
#include "zxmacros.h"
#include "view_templates.h"

#include <string.h>
#include <stdio.h>

#if defined(TARGET_NANOS)

void rs_h_expert_toggle();
void rs_h_expert_update();
void rs_h_review_button_left();
void rs_h_review_button_right();
void rs_h_review_button_both();

bool rs_h_paging_can_decrease(void);
bool rs_h_paging_can_increase(void);

ux_state_t ux;

void os_exit(uint32_t id) {
    (void)id;
    os_sched_exit(0);
}

//Referenced in crapoline_ux_menu_display
const ux_menu_entry_t menu_main[] = {
    {NULL, NULL, 0, &C_icon_app, MENU_MAIN_APP_LINE1, BACKEND_LAZY.key, 33, 12},
    {NULL, rs_h_expert_toggle, 0, &C_icon_app, "Expert mode:", BACKEND_LAZY.value, 33, 12},
    {NULL, NULL, 0, &C_icon_app, APPVERSION_LINE1, APPVERSION_LINE2, 33, 12},

    {NULL,
     NULL,
     0, &C_icon_app, "Developed by:", "Zondax.ch", 33, 12},

    {NULL, NULL, 0, &C_icon_app, "License: ", "Apache 2.0", 33, 12},
    {NULL, os_exit, 0, &C_icon_dashboard, "Quit", NULL, 50, 29},
    UX_MENU_END
};

//Referenced in crapoline_ux_display_view_review
static const bagl_element_t view_review[] = {
    UI_BACKGROUND_LEFT_RIGHT_ICONS,
    UI_LabelLine(UIID_LABEL + 0, 0, 8, UI_SCREEN_WIDTH, UI_11PX, UI_WHITE, UI_BLACK, BACKEND_LAZY.key),
    UI_LabelLine(UIID_LABEL + 1, 0, 19, UI_SCREEN_WIDTH, UI_11PX, UI_WHITE, UI_BLACK, BACKEND_LAZY.value),
    UI_LabelLine(UIID_LABEL + 2, 0, 30, UI_SCREEN_WIDTH, UI_11PX, UI_WHITE, UI_BLACK, BACKEND_LAZY.value2),
};

//Referenced in crapoline_ux_display_view_error
static const bagl_element_t view_error[] = {
    UI_FillRectangle(0, 0, 0, UI_SCREEN_WIDTH, UI_SCREEN_HEIGHT, 0x000000, 0xFFFFFF),
    UI_Icon(0, 128 - 7, 0, 7, 7, BAGL_GLYPH_ICON_CHECK),
    UI_LabelLine(UIID_LABEL + 0, 0, 8, UI_SCREEN_WIDTH, UI_11PX, UI_WHITE, UI_BLACK, BACKEND_LAZY.key),
    UI_LabelLine(UIID_LABEL + 0, 0, 19, UI_SCREEN_WIDTH, UI_11PX, UI_WHITE, UI_BLACK, BACKEND_LAZY.value),
    UI_LabelLineScrolling(UIID_LABELSCROLL, 0, 30, 128, UI_11PX, UI_WHITE, UI_BLACK, BACKEND_LAZY.value2),
};

//Referenced by crapoline_ux_display_view_error macro call
static unsigned int view_error_button(unsigned int button_mask, unsigned int button_mask_counter) {
    UNUSED(button_mask_counter);
    switch (button_mask) {
        case BUTTON_EVT_RELEASED | BUTTON_LEFT | BUTTON_RIGHT:
        case BUTTON_EVT_RELEASED | BUTTON_LEFT:
            break;
        case BUTTON_EVT_RELEASED | BUTTON_RIGHT:
            rs_h_error_accept(0);
            break;
    }
    return 0;
}

//Referenced by crapoline_ux_display_view_review macro call
static unsigned int view_review_button(unsigned int button_mask, unsigned int button_mask_counter) {
    UNUSED(button_mask_counter);
    switch (button_mask) {
        case BUTTON_EVT_RELEASED | BUTTON_LEFT | BUTTON_RIGHT:
            rs_h_review_button_both();
            break;
        case BUTTON_EVT_RELEASED | BUTTON_LEFT:
            // Press left to progress to the previous element
            rs_h_review_button_left();
            break;

        case BUTTON_EVT_RELEASED | BUTTON_RIGHT:
            // Press right to progress to the next element
            rs_h_review_button_right();
            break;
    }
    return 0;
}

const bagl_element_t *view_prepro(const bagl_element_t *element) {
    switch (element->component.userid) {
        case UIID_ICONLEFT:
            if (!rs_h_paging_can_decrease()){
                return NULL;
            }
            UX_CALLBACK_SET_INTERVAL(2000);
            break;
        case UIID_ICONRIGHT:
            if (!rs_h_paging_can_increase()){
                return NULL;
            }
            UX_CALLBACK_SET_INTERVAL(2000);
            break;
        case UIID_LABELSCROLL:
            UX_CALLBACK_SET_INTERVAL(
                MAX(3000, 1000 + bagl_label_roundtrip_duration_ms(element, 7))
            );
            break;
    }
    return element;
}

const bagl_element_t *view_prepro_idle(const bagl_element_t *element) {
    switch (element->component.userid) {
        case UIID_ICONLEFT:
        case UIID_ICONRIGHT:
            return NULL;
    }
    return element;
}

//////////////////////////
//////////////////////////
//////////////////////////
//////////////////////////
//////////////////////////

//// VIEW MESSAGE

static const bagl_element_t view_message[] = {
    UI_BACKGROUND,
    UI_LabelLine(UIID_LABEL + 0, 0, 8, UI_SCREEN_WIDTH, UI_11PX, UI_WHITE, UI_BLACK, BACKEND_LAZY.key),
    UI_LabelLine(UIID_LABEL + 1, 0, 19, UI_SCREEN_WIDTH, UI_11PX, UI_WHITE, UI_BLACK, BACKEND_LAZY.value),
};

static unsigned int view_message_button(unsigned int button_mask, unsigned int button_mask_counter) {
    UNUSED(button_mask_counter);
    switch (button_mask) {
        case BUTTON_EVT_RELEASED | BUTTON_LEFT | BUTTON_RIGHT:
        case BUTTON_EVT_RELEASED | BUTTON_LEFT:
        case BUTTON_EVT_RELEASED | BUTTON_RIGHT:
            break;
    }
    return 0;
}

/********* CRAPOLINES *************/

void crapoline_ux_wait() {
    UX_WAIT();
}

void crapoline_ux_menu_display(uint8_t item_idx) {
    //menu_main is ux_menu_t above
    UX_MENU_DISPLAY(item_idx, menu_main, NULL);
}

void crapoline_ux_display_view_error() {
    UX_DISPLAY(view_error, view_prepro);
}

void crapoline_ux_display_view_review() {
    UX_DISPLAY(view_review, view_prepro);
}

void crapoline_ux_display_view_message() {
    UX_DISPLAY(view_message, view_prepro_idle);
}
#endif
