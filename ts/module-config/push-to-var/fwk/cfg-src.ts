/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

import { makeMutableFunctionArity0, MutableFunctionArity0 } from "../../../arch/util/mutable-function-arity-0";

export type CfgSrc<T> = MutableFunctionArity0<T>

export function makeCfgSrc<T>(src: (() => T) | null): CfgSrc<T> {
  return makeMutableFunctionArity0(src)
}

type CfgSrcAdapter<S, T> = (s: S) => T

export class CfgSrcAdaptation<S, T> {
  constructor(
    private targetSrc: CfgSrc<T>,
    private adapter: CfgSrcAdapter<S, T>
  ) {
  }

  setOrigin(originSrc: () => S) {
    this.targetSrc.set(() => this.adapter(originSrc()))
  }
}
