/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

import { fooSflCfgSrc, fooSfl } from "../fs/foo-sfl";
import { barBfCfgSrc } from "../fs/bar-bf";

fooSflCfgSrc.set(() => {
  return { x: "foo" }
})

barBfCfgSrc.set(() => {
  return { z: 99 }
})

fooSfl()
