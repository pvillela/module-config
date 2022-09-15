/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

import { makeMutableFunctionArity1, MutableFunctionArity1 } from "../../arch/util/mutable-function-arity-1";
import { liftToNullary } from "../../arch/util/lift-to-nullary";

export function mutableLifted<S, T>(f: (s: S) => T): MutableFunctionArity1<(() => S) | null, () => T> {
  return makeMutableFunctionArity1(liftToNullary(f))
}
