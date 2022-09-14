/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

import { AppCfgInfo } from "../../config/app-cfg";
import { barBfCfgSrc, BarBfCfgInfo } from "../bar-bf";
import { CfgSrcAdaptation } from "../../fwk/cfg-src";

function adapter(appCfg: AppCfgInfo): BarBfCfgInfo {
  return {
    z: appCfg.y
  };
}

export const barBfCfgAdaptation = new CfgSrcAdaptation(
  barBfCfgSrc,
  adapter,
)
