/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

import { makeCfgSrc } from "../fwk/cfg-src";

export type BarBfCfgInfo = {
  z: number;
};

export const barBfCfgSrc = makeCfgSrc<BarBfCfgInfo>(null);

export function barBf() {
  console.log(barBfCfgSrc().z);
}
