/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

import { ConfigurationError } from "./configuration-error";

export function liftToNullary<S, T>(f: (s: S) => T): (sSrc: (() => S) | null) => () => T {
  return (sSrc) => {
    return () => {
      if (sSrc == null) {
        throw new ConfigurationError("Null configuration source")
      }
      return f(sSrc())
    }
  }
}
