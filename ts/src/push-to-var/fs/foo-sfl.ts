/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

import { makeCfgSrc } from "../fwk/cfg-src";
import { barBf } from "./bar-bf";

export type FooSflCfgInfo = {
  x: string;
};

export const fooSflCfgSrc = makeCfgSrc<FooSflCfgInfo>(null);

export function fooSfl() {
  console.log(fooSflCfgSrc().x);
  barBf()
}
