/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

import { ConfigurationError } from "./configuration-error";

export interface MutableFunctionArity0<T> {
  (): T
  set(f: () => T): void
}

export function nilArity0<T>(): T {
  throw new ConfigurationError("Module used before being initialized.")
}

class MutableFunctionArity0Helper<T> {
  constructor(
    private f: () => T
  ) {
  }
  set(src: () => T) {
    this.f = src
  }

  call(): T {
    return this.f()
  }
}

export function makeMutableFunctionArity0<T>(
  f: (() => T) | null
): MutableFunctionArity0<T> {
  const f1 = f || nilArity0
  const instance = new MutableFunctionArity0Helper<T>(f1)
  return Object.assign(
    () => instance.call(),
    {
      set: (src: () => T) => instance.set(src)
    }
  )
}
