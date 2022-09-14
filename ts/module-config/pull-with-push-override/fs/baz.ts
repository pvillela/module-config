/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

import { makeCfgSrc } from "../config/cfg-src";

type BazCfgInfo = {
  w: string;
};

export const bazCfgSrc = makeCfgSrc<BazCfgInfo>(null);

export function baz() {
  console.log(bazCfgSrc().w.length);
}
