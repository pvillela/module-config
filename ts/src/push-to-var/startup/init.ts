/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

import { getAppConfiguration as c } from "../config/app-cfg";
import { barBfCfgAdaptation } from "../fs/cfg-adapt/bar-bf-cfg-adapter"
import { fooSflCfgAdaptation } from "../fs/cfg-adapt/FooSflCfgAdapter";

fooSflCfgAdaptation.setOrigin(c);
barBfCfgAdaptation.setOrigin(c);
