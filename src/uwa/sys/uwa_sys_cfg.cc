/******************************************************************************
 *
 *  Copyright (C) 1999-2012 Broadcom Corporation
 *  Copyright 2018-2019 NXP
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at:
 *
 *  http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *
 ******************************************************************************/
/******************************************************************************
 *
 *  This file contains compile-time configurable constants for the UWA
 *  system manager.
 *
 ******************************************************************************/

#include "uwb_gki.h"
#include "uwa_sys.h"

const tUWA_SYS_CFG uwa_sys_cfg = {
    UWA_MBOX_EVT_MASK, /* GKI mailbox event */
    UWA_MBOX_ID,       /* GKI mailbox id */
    UWA_TIMER_ID,      /* GKI timer id */
};

tUWA_SYS_CFG* p_uwa_sys_cfg = (tUWA_SYS_CFG*)&uwa_sys_cfg;
