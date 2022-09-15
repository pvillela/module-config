/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

import { barBfCfgAdapter } from "./bar-bf-cfg-adapter";
import { makeCfgSrc } from "../config/cfg-src";

export type BarBfCfgInfo = {
  z: number;
};

export const barBfCfgSrc = makeCfgSrc<BarBfCfgInfo>(barBfCfgAdapter)

export function barBf() {
  console.log(barBfCfgSrc().z);
}
