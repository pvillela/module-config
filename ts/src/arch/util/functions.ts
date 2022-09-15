/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

export function thunkOf<T>(t: T): () => T {
  return () => t
}

export function constantOf<S, T>(t: T): (s: S) => T {
  return (s: S) => t
}
