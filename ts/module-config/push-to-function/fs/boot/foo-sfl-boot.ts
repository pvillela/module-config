/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

import { AppCfgInfo} from "../../config/app-cfg";
import { fooSflC, FooSflCfgInfo, FooSflT } from "../foo-sfl";
import { barBfBoot } from "./bar-bf-boot";
import { mutableLifted } from "../../fwk/mutable-lifted";

function adapter(appCfgInfo: AppCfgInfo): FooSflCfgInfo {
  return appCfgInfo
}

export const fooSflCfgAdapter = mutableLifted(adapter)

export function fooSflBoot(appCfgSrc: () => AppCfgInfo): FooSflT {
  const cfgSrc = {
    get: fooSflCfgAdapter(appCfgSrc),
    bar: barBfBoot(appCfgSrc)
  }
  return fooSflC(cfgSrc)
}
