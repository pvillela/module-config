/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

import { ConfigurationError } from "./configuration-error";

export interface MutableFunctionArity1<S, T> {
  (s: S): T
  set(f: (s: S) => T): void
}

function nilArity1<S, T>(s: S): T {
  throw new ConfigurationError("Module used before being initialized.")
}

class MutableFunctionArity1Helper<S, T> {
  constructor(
    private f: (s: S) => T
  ) {
  }
  set(f: (s: S) => T) {
    this.f = f
  }

  call(s: S): T {
    return this.f(s)
  }
}

export function makeMutableFunctionArity1<S, T>(
  f: ((s: S) => T) | null
): MutableFunctionArity1<S, T> {
  const f1 = f || nilArity1
  const instance = new MutableFunctionArity1Helper<S, T>(f1)
  return Object.assign(
    (s: S) => instance.call(s),
    {
      set: (src: (s: S) => T) => instance.set(src)
    }
  )
}
