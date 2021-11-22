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
#include "actions.h"
#include "glyphs.h"
#include "bagl.h"
#include "zxmacros.h"
#include "view_templates.h"

#include <string.h>
#include <stdio.h>

#if defined(TARGET_NANOX)

void rs_h_expert_toggle();
void rs_h_expert_update();

void rs_h_review_loop_start();
void rs_h_review_loop_inside();
void rs_h_review_loop_end();

void rs_h_approve(unsigned int);
void rs_h_reject(unsigned int);

#include "ux.h"
ux_state_t G_ux;
bolos_ux_params_t G_ux_params;
uint8_t flow_inside_loop;


UX_STEP_NOCB(ux_idle_flow_1_step, pbb, { &C_icon_app, MENU_MAIN_APP_LINE1, BACKEND_LAZY.key,});
UX_STEP_CB_INIT(ux_idle_flow_2_step, bn,  rs_h_expert_update(), rs_h_expert_toggle(), { "Expert mode:", BACKEND_LAZY.message, });
UX_STEP_NOCB(ux_idle_flow_3_step, bn, { APPVERSION_LINE1, APPVERSION_LINE2, });
UX_STEP_NOCB(ux_idle_flow_4_step, bn, { "Developed by:", "Zondax.ch", });
UX_STEP_NOCB(ux_idle_flow_5_step, bn, { "License:", "Apache 2.0", });
UX_STEP_CB(ux_idle_flow_6_step, pb, os_sched_exit(-1), { &C_icon_dashboard, "Quit",});

const ux_flow_step_t *const ux_idle_flow [] = {
  &ux_idle_flow_1_step,
  &ux_idle_flow_2_step,
  &ux_idle_flow_3_step,
  &ux_idle_flow_4_step,
  &ux_idle_flow_5_step,
  &ux_idle_flow_6_step,
  FLOW_END_STEP,
};

///////////

UX_STEP_NOCB(ux_error_flow_1_step, bnnn_paging, { .title = BACKEND_LAZY.key, .text = BACKEND_LAZY.message, });
UX_STEP_VALID(ux_error_flow_2_step, pb, rs_h_error_accept(0), { &C_icon_validate_14, "Ok"});

UX_FLOW(
    ux_error_flow,
    &ux_error_flow_1_step,
    &ux_error_flow_2_step
);

///////////

UX_FLOW_DEF_NOCB(ux_review_flow_1_review_title, pbb, { &C_icon_app, "Please", "review",});
UX_STEP_INIT(ux_review_flow_2_start_step, NULL, NULL, { rs_h_review_loop_start(); });
UX_STEP_NOCB_INIT(ux_review_flow_2_step, bnnn_paging, { rs_h_review_loop_inside(); }, { .title = BACKEND_LAZY.key, .text = BACKEND_LAZY.message, });
UX_STEP_INIT(ux_review_flow_2_end_step, NULL, NULL, { rs_h_review_loop_end(); });
UX_STEP_VALID(ux_review_flow_3_step, pb, rs_h_approve(0), { &C_icon_validate_14, APPROVE_LABEL });
UX_STEP_VALID(ux_review_flow_4_step, pb, rs_h_reject(0), { &C_icon_crossmark, REJECT_LABEL });

const ux_flow_step_t *const ux_review_flow[] = {
  &ux_review_flow_1_review_title,
  &ux_review_flow_2_start_step,
  &ux_review_flow_2_step,
  &ux_review_flow_2_end_step,
  &ux_review_flow_3_step,
  &ux_review_flow_4_step,
  FLOW_END_STEP,
};

//////////////////////////
//////////////////////////
//////////////////////////
//////////////////////////
//////////////////////////

/********* CRAPOLINES *************/

void crapoline_ux_wait() {
    UX_WAIT();
}

void crapoline_ux_flow_init_idle_flow_toggle_expert() {
   ux_flow_init(0, ux_idle_flow, &ux_idle_flow_2_step);
}

void crapoline_ux_show_review() {
    if (G_ux.stack_count == 0) {
        ux_stack_push();
    }

    ux_flow_init(0, ux_review_flow, NULL);
}

void crapoline_ux_show_error() {
    ux_layout_bnnn_paging_reset();

    if (G_ux.stack_count == 0) {
        ux_stack_push();
    }

    ux_flow_init(0, ux_error_flow, NULL);
}

void crapoline_ux_show_idle() {
    if(G_ux.stack_count == 0) {
        ux_stack_push();
    }

    ux_flow_init(0, ux_idle_flow, NULL);
}

void crapoline_ux_flow_prev() {
    ux_flow_prev();
}

void crapoline_ux_flow_next() {
    ux_flow_next();
}

void crapoline_ux_layout_bnnn_paging_reset() {
    ux_layout_bnnn_paging_reset();
}

void crapoline_ux_flow_relayout() {
    // move to prev flow but trick paging to show first page

    CUR_FLOW.prev_index = CUR_FLOW.index-2;
    CUR_FLOW.index--;
    ux_flow_relayout();
}
#endif
