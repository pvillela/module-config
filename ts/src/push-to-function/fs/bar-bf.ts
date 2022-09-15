/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

export type BarBfCfgInfo = {
  z: number;
};

export type BarBfCfgSrc = () => BarBfCfgInfo

export type BarBfT = () => void;

export function barBfC(cfg: BarBfCfgSrc): BarBfT {
  return function() {
    console.log(cfg().z);
  }
}
