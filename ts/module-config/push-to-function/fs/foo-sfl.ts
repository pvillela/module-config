/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

export type FooSflCfgInfo = {
  x: string;
};

export type FooSflCfgSrc = {
  get: () => FooSflCfgInfo;
  bar: () => void;
};

export type FooSflT = () => void;

export function fooSflC(cfg: FooSflCfgSrc): FooSflT {
  return function() {
    console.log(cfg.get().x);
    cfg.bar()
  }
}
