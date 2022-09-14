/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

import { barBf } from "./bar-bf";
import { identity, makeCfgSrc } from "../config/cfg-src";

type FooSflCfgInfo = {
  x: string;
};

export const fooSflCfgSrc = makeCfgSrc<FooSflCfgInfo>(identity)

export function fooSfl() {
  console.log(fooSflCfgSrc().x);
  barBf();
}
