/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

import { AppCfgInfo} from "../../config/app-cfg";
import { barBfC, BarBfT, BarBfCfgInfo } from "../bar-bf";
import { mutableLifted } from "../../fwk/mutable-lifted";

function adapter(appCfg: AppCfgInfo): BarBfCfgInfo {
  return {
    z: appCfg.y
  };
}

export const barBfCfgAdapter = mutableLifted(adapter)

export function barBfBoot(appCfg: () => AppCfgInfo): BarBfT {
  return barBfC(barBfCfgAdapter(appCfg))
}
